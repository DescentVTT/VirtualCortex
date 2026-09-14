//! A compaction of the term arena (whitepaper §5.2.30, §8.6, §8.8; ADR-0056): the nodes a set
//! of roots does not reach are reclaimed, in two passes over the arena below its cursor. The
//! arena is bottom-up by construction (a child's index is below its parent's: the
//! constructors, the operators, the instantiation and the loader all keep it), so one
//! descending pass marks every live node from the roots (a marked node marks its children,
//! which lie below it and are visited after it), and one ascending pass moves every live node
//! to the next free slot, remapping its children through the table the pass fills (a child is
//! moved before its parent). The roots are remapped in place, the dead tail is zeroed, and
//! the live nodes keep their order, so the compacted arena is bottom-up too. The bindings are
//! not an input: a caller compacts an arena whose binding table is empty (the executor's is,
//! between searches), since a binding names an index. Nothing here allocates, and every loop
//! is a range over the cursor or a slice's iterator.

use crate::term::TermNode;

/// Why a compaction was refused. Nothing changes on a refusal: the checks run before the
/// first write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompactError {
    /// A root at or beyond the cursor.
    Root,
    /// A child at or beyond its parent: the arena is not bottom-up, so one pass could not
    /// mark it.
    NotBottomUp,
    /// The scratch is shorter than the cursor, or the cursor is beyond the arena.
    Scratch,
}

/// What a compaction did.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Compaction {
    /// The nodes kept: the arena's cursor after.
    pub live: u32,
    /// The nodes reclaimed.
    pub reclaimed: u32,
}

/// The scratch's word for a node the roots do not reach.
const DEAD: u32 = u32::MAX;
/// The scratch's word for a marked node not yet moved; a moved node's word is its new index,
/// which is below the cursor and so below both sentinels.
const LIVE: u32 = u32::MAX - 1;

/// Compacts `arena[..free]` onto the nodes `roots` reach: the mark, the move, the roots
/// remapped in place, the tail zeroed. `forward` is the caller's scratch of at least `free`
/// entries (its contents are the rule's after). Returns the nodes kept and reclaimed;
/// refused, with nothing changed, for a root at or beyond `free`, a child at or beyond its
/// parent, a scratch shorter than `free` or a cursor beyond the arena. A compaction of an
/// arena that holds no garbage moves nothing and reports nothing reclaimed.
pub fn compact(
    arena: &mut [TermNode],
    free: usize,
    roots: &mut [u32],
    forward: &mut [u32],
) -> Result<Compaction, CompactError> {
    let Some(nodes) = arena.get_mut(..free) else {
        return Err(CompactError::Scratch);
    };
    let Some(table) = forward.get_mut(..free) else {
        return Err(CompactError::Scratch);
    };
    if roots.iter().any(|&root| root as usize >= free) {
        return Err(CompactError::Root);
    }
    let bottom_up = nodes.iter().enumerate().all(|(index, node)| {
        (0..node.arity as usize).all(|slot| match node.child(slot) {
            Some(child) => (child as usize) < index,
            None => true,
        })
    });
    if !bottom_up {
        return Err(CompactError::NotBottomUp);
    }
    // The mark: the roots, then, descending, every child of a marked node. A child lies
    // below its parent, so it is visited after the parent marked it.
    table.fill(DEAD);
    for &root in roots.iter() {
        table[root as usize] = LIVE;
    }
    for index in (0..free).rev() {
        if table[index] != LIVE {
            continue;
        }
        let node = nodes[index];
        for slot in 0..node.arity as usize {
            if let Some(child) = node.child(slot) {
                table[child as usize] = LIVE;
            }
        }
    }
    // The move: ascending, a live node takes the next free slot, which is at or below its
    // own, so nothing live is overwritten before it is read; its children, moved before
    // it, are remapped through the table.
    let mut next = 0usize;
    for index in 0..free {
        if table[index] != LIVE {
            continue;
        }
        let mut node = nodes[index];
        for slot in 0..node.arity as usize {
            if let Some(child) = node.child(slot) {
                // A child is marked and moved: its word is its new index, below the cursor,
                // so the encoding cannot wrap.
                node.children[slot] = table[child as usize].wrapping_add(1);
            }
        }
        // `next` is below `free`, which is below `u32::MAX` by the encoding of a child.
        table[index] = next as u32;
        nodes[next] = node;
        next = next.wrapping_add(1);
    }
    if let Some(tail) = nodes.get_mut(next..) {
        for node in tail {
            *node = TermNode::default();
        }
    }
    for root in roots.iter_mut() {
        *root = table[*root as usize];
    }
    Ok(Compaction {
        live: next as u32,
        // `next` is at most `free`: every kept node was one of the `free`.
        reclaimed: free.wrapping_sub(next) as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::induce::{size, term_hash};
    use crate::term::{Binding, TERM_NONE};

    /// A measure of `term` in `arena` with no binding: `(size, hash)`.
    fn measure(term: u32, arena: &[TermNode]) -> (u32, u32) {
        let bindings = [Binding::UNBOUND; 64];
        let mut stack = [0u32; 512];
        (
            size(term, arena, &bindings, &mut stack).unwrap(),
            term_hash(term, arena, &bindings, &mut stack).unwrap(),
        )
    }

    #[test]
    fn garbage_below_between_and_above_the_live_nodes_is_reclaimed_and_the_roots_remapped() {
        // 0 dead constant, 1 a, 2 dead, 3 X, 4 f(a, X), 5 dead, 6 dead g(dead), 7 h(f(a, X)),
        // 8 dead: the root 7 reaches 1, 3, 4 and itself.
        let mut arena = [TermNode::default(); 10];
        arena[0] = TermNode::constant(9);
        arena[1] = TermNode::constant(1);
        arena[2] = TermNode::variable(7);
        arena[3] = TermNode::variable(0);
        arena[4] = TermNode::compound(0x10, &[1, 3]).unwrap();
        arena[5] = TermNode::constant(5);
        arena[6] = TermNode::compound(0x11, &[5]).unwrap();
        arena[7] = TermNode::compound(0x12, &[4]).unwrap();
        arena[8] = TermNode::constant(8);
        let before = measure(7, &arena);
        let mut roots = [7u32];
        let mut scratch = [0u32; 9];
        let report = compact(&mut arena, 9, &mut roots, &mut scratch).unwrap();
        assert_eq!(
            report,
            Compaction {
                live: 4,
                reclaimed: 5
            }
        );
        assert_eq!(roots, [3]);
        assert_eq!(
            arena[..4],
            [
                TermNode::constant(1),
                TermNode::variable(0),
                TermNode::compound(0x10, &[0, 1]).unwrap(),
                TermNode::compound(0x12, &[2]).unwrap(),
            ],
            "the live nodes in their order, the children remapped"
        );
        assert_eq!(&arena[4..], &[TermNode::default(); 6], "the tail is zero");
        assert_eq!(measure(3, &arena), before, "the root reads the same");
        assert_eq!(arena[2].children[2], TERM_NONE);
        // Nothing left to reclaim: a second compaction moves nothing.
        assert_eq!(
            compact(&mut arena, 4, &mut roots, &mut scratch).unwrap(),
            Compaction {
                live: 4,
                reclaimed: 0
            }
        );
        assert_eq!(roots, [3]);
    }

    #[test]
    fn a_subterm_two_roots_share_is_kept_once_and_a_store_of_no_roots_empties_the_arena() {
        // 0 a, 1 dead, 2 f(a), 3 g(f(a)), 4 h(f(a), a): the roots 3 and 4 share 2 and 0.
        let mut arena = [TermNode::default(); 6];
        arena[0] = TermNode::constant(1);
        arena[1] = TermNode::constant(2);
        arena[2] = TermNode::compound(0x10, &[0]).unwrap();
        arena[3] = TermNode::compound(0x11, &[2]).unwrap();
        arena[4] = TermNode::compound(0x12, &[2, 0]).unwrap();
        let (b3, b4) = (measure(3, &arena), measure(4, &arena));
        let mut roots = [4u32, 3, 4];
        let mut scratch = [0u32; 8];
        assert_eq!(
            compact(&mut arena, 5, &mut roots, &mut scratch).unwrap(),
            Compaction {
                live: 4,
                reclaimed: 1
            }
        );
        assert_eq!(roots, [3, 2, 3], "a root named twice is remapped twice");
        assert_eq!((measure(2, &arena), measure(3, &arena)), (b3, b4));
        assert_eq!(arena[1], TermNode::compound(0x10, &[0]).unwrap());
        assert_eq!(arena[3], TermNode::compound(0x12, &[1, 0]).unwrap());
        // No root: everything is garbage.
        let mut none: [u32; 0] = [];
        assert_eq!(
            compact(&mut arena, 4, &mut none, &mut scratch).unwrap(),
            Compaction {
                live: 0,
                reclaimed: 4
            }
        );
        assert_eq!(arena, [TermNode::default(); 6]);
        // An empty arena compacts to nothing.
        assert_eq!(
            compact(&mut arena, 0, &mut none, &mut scratch).unwrap(),
            Compaction::default()
        );
    }

    #[test]
    fn every_refusal_leaves_the_arena_the_roots_and_the_scratch_as_they_were() {
        let mut arena = [TermNode::default(); 4];
        arena[0] = TermNode::constant(1);
        arena[1] = TermNode::compound(0x10, &[0]).unwrap();
        arena[2] = TermNode::constant(2);
        let before = arena;
        let mut roots = [1u32, 2];
        let mut scratch = [7u32; 4];
        assert_eq!(
            compact(&mut arena, 3, &mut [3u32], &mut scratch),
            Err(CompactError::Root),
            "a root at the cursor"
        );
        assert_eq!(
            compact(&mut arena, 3, &mut roots, &mut scratch[..2]),
            Err(CompactError::Scratch),
            "a scratch shorter than the cursor"
        );
        assert_eq!(
            compact(&mut arena, 5, &mut roots, &mut [0u32; 8]),
            Err(CompactError::Scratch),
            "a cursor beyond the arena"
        );
        let mut forward = arena;
        forward[1] = TermNode::compound(0x10, &[2]).unwrap();
        assert_eq!(
            compact(&mut forward, 3, &mut roots, &mut scratch),
            Err(CompactError::NotBottomUp),
            "a child above its parent"
        );
        let mut own = arena;
        own[2] = TermNode::compound(0x10, &[2]).unwrap();
        assert_eq!(
            compact(&mut own, 3, &mut roots, &mut scratch),
            Err(CompactError::NotBottomUp),
            "a child at its parent"
        );
        assert_eq!(arena, before);
        assert_eq!(roots, [1, 2]);
        assert_eq!(scratch, [7; 4], "a refusal writes nothing");
        // A root beyond a dead node is fine; the roots are remapped.
        assert_eq!(
            compact(&mut arena, 3, &mut roots, &mut scratch).unwrap(),
            Compaction {
                live: 3,
                reclaimed: 0
            }
        );
        assert_eq!(roots, [1, 2]);
    }
}

/// Property tests (ADR-0030): over seeded bottom-up arenas with seeded roots, every root
/// reads the same after a compaction (its size and its hash through no binding), the kept and
/// the reclaimed nodes sum to the cursor, the compacted arena is bottom-up and a second
/// compaction reclaims nothing.
#[cfg(test)]
mod prop {
    use super::*;
    use crate::induce::{size, term_hash};
    use crate::term::{Binding, MAX_ARITY};
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testkit/prop.rs"
    ));

    const NODES: usize = 64;

    fn measure(term: u32, arena: &[TermNode]) -> (u32, u32) {
        let bindings = [Binding::UNBOUND; NODES];
        let mut stack = [0u32; 4 * NODES];
        (
            size(term, arena, &bindings, &mut stack).unwrap(),
            term_hash(term, arena, &bindings, &mut stack).unwrap(),
        )
    }

    fn is_bottom_up(arena: &[TermNode], free: usize) -> bool {
        arena[..free].iter().enumerate().all(|(index, node)| {
            (0..node.arity as usize)
                .all(|slot| node.child(slot).is_some_and(|c| (c as usize) < index))
        })
    }

    #[test]
    fn a_seeded_arena_reads_the_same_at_every_root_after_a_compaction_and_a_second_reclaims_nothing()
     {
        let mut rng = Lcg::new(0x5EED_C0DE);
        for _ in 0..2_000 {
            let free = 1 + rng.below(NODES as u32) as usize;
            let mut arena = [TermNode::default(); NODES];
            for (index, node) in arena[..free].iter_mut().enumerate() {
                *node = match rng.below(3) {
                    0 => TermNode::constant(rng.below(100)),
                    1 => TermNode::variable(rng.below(NODES as u32)),
                    _ if index == 0 => TermNode::constant(rng.below(100)),
                    _ => {
                        let arity = 1 + rng.below(MAX_ARITY as u32) as usize;
                        let mut args = [0u32; MAX_ARITY];
                        for arg in &mut args[..arity] {
                            *arg = rng.below(index as u32);
                        }
                        TermNode::compound(0x100 + rng.below(16), &args[..arity]).unwrap()
                    }
                };
            }
            let count = rng.below(5) as usize;
            let mut roots = [0u32; 4];
            for root in &mut roots[..count] {
                *root = rng.below(free as u32);
            }
            let mut before = [(0u32, 0u32); 4];
            for (slot, &root) in roots[..count].iter().enumerate() {
                before[slot] = measure(root, &arena);
            }
            let mut scratch = [0u32; NODES];
            let report = compact(&mut arena, free, &mut roots[..count], &mut scratch).unwrap();
            assert_eq!(
                report.live.wrapping_add(report.reclaimed) as usize,
                free,
                "the kept and the reclaimed are the cursor"
            );
            assert!(report.live as usize <= free);
            assert!(is_bottom_up(&arena, report.live as usize));
            for (root, expected) in roots[..count].iter().zip(&before[..count]) {
                assert!((*root as usize) < report.live as usize);
                assert_eq!(measure(*root, &arena), *expected, "a root reads the same");
            }
            assert!(
                arena[report.live as usize..]
                    .iter()
                    .all(|n| *n == TermNode::default()),
                "the tail is zero"
            );
            let again = compact(
                &mut arena,
                report.live as usize,
                &mut roots[..count],
                &mut scratch,
            )
            .unwrap();
            assert_eq!(
                again,
                Compaction {
                    live: report.live,
                    reclaimed: 0
                },
                "idempotent"
            );
            if count == 0 {
                assert_eq!(report.live, 0, "no root keeps nothing");
            }
        }
    }
}
