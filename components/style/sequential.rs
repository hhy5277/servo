/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Implements sequential traversal over the DOM tree.

use dom::TNode;
use traversal::DomTraversalContext;

pub fn traverse_dom<N, C>(root: N,
                          shared: &C::SharedContext)
    where N: TNode,
          C: DomTraversalContext<N>
{
    fn doit<'a, N, C>(context: &'a C, node: N)
        where N: TNode,
              C: DomTraversalContext<N>
    {
        context.process_preorder(node);
        if let Some(el) = node.as_element() {
            C::traverse_children(el, |kid| doit::<N, C>(context, kid));
        }

        if context.needs_postorder_traversal() {
            context.process_postorder(node);
        }
    }

    let context = C::new(shared, root.opaque());
    doit::<N, C>(&context, root);

    // Clear the local LRU cache since we store stateful elements inside.
    context.local_context().style_sharing_candidate_cache.borrow_mut().clear();
}
