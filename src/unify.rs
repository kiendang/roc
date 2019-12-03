use crate::subs::Content::{self, *};
use crate::subs::{Descriptor, FlatType, Mark, Subs, Variable};
use crate::types::Problem;

struct Context {
    first: Variable,
    first_desc: Descriptor,
    second: Variable,
    second_desc: Descriptor,
}

#[inline(always)]
pub fn unify(subs: &mut Subs, var1: Variable, var2: Variable) {
    if !subs.equivalent(var1, var2) {
        let ctx = Context {
            first: var1,
            first_desc: subs.get(var1),
            second: var2,
            second_desc: subs.get(var2),
        };

        unify_context(subs, &ctx)
    }
}

fn unify_context(subs: &mut Subs, ctx: &Context) {
    match ctx.first_desc.content {
        FlexVar(ref opt_name) => unify_flex(subs, ctx, opt_name, &ctx.second_desc.content),
        RigidVar(ref name) => unify_rigid(subs, ctx, name, &ctx.second_desc.content),
        Structure(ref flat_type) => unify_structure(subs, ctx, flat_type, &ctx.second_desc.content),
        Error(ref problem) => {
            // Error propagates. Whatever we're comparing it to doesn't matter!
            merge(subs, ctx, Error(problem.clone()))
        }
    }
}

#[inline(always)]
fn unify_structure(subs: &mut Subs, ctx: &Context, flat_type: &FlatType, other: &Content) {
    match other {
        FlexVar(_) => {
            // If the other is flex, Structure wins!
            merge(subs, ctx, Structure(flat_type.clone()))
        }
        RigidVar(_) => {
            // Type mismatch! Rigid can only unify with flex.
            merge(subs, ctx, Error(Problem::GenericMismatch))
        }
        Structure(ref other_flat_type) => {
            // Unify the two flat types
            unify_flat_type(subs, ctx, flat_type, other_flat_type)
        }
        Error(problem) => {
            // Error propagates.
            merge(subs, ctx, Error(problem.clone()))
        }
    }
}

#[inline(always)]
fn unify_flat_type(subs: &mut Subs, ctx: &Context, left: &FlatType, right: &FlatType) {
    use crate::subs::FlatType::*;

    match (left, right) {
        (EmptyRecord, EmptyRecord) => merge(subs, ctx, Structure(left.clone())),
        (
            Apply {
                module_name: l_module_name,
                name: l_type_name,
                args: l_args,
            },
            Apply {
                module_name: r_module_name,
                name: r_type_name,
                args: r_args,
            },
        ) if l_module_name == r_module_name && l_type_name == r_type_name => {
            unify_zip(subs, l_args.iter(), r_args.iter());

            merge(
                subs,
                ctx,
                Structure(Apply {
                    module_name: (*r_module_name).clone(),
                    name: (*r_type_name).clone(),
                    args: (*r_args).clone(),
                }),
            )
        }
        (Func(l_args, l_ret), Func(r_args, r_ret)) => {
            if l_args.len() == r_args.len() {
                unify_zip(subs, l_args.iter(), r_args.iter());
                unify(subs, *l_ret, *r_ret);

                merge(subs, ctx, Structure(Func((*r_args).clone(), *r_ret)))
            } else if l_args.len() > r_args.len() {
                merge(subs, ctx, Error(Problem::ExtraArguments))
            } else {
                merge(subs, ctx, Error(Problem::MissingArguments))
            }
        }
        _ => merge(subs, ctx, Error(Problem::GenericMismatch)),
    }
}

fn unify_zip<'a, I>(subs: &mut Subs, left_iter: I, right_iter: I)
where
    I: Iterator<Item = &'a Variable>,
{
    for (&l_var, &r_var) in left_iter.zip(right_iter) {
        unify(subs, l_var, r_var);
    }
}

#[inline(always)]
fn unify_rigid(subs: &mut Subs, ctx: &Context, name: &str, other: &Content) {
    match other {
        FlexVar(_) => {
            // If the other is flex, rigid wins!
            merge(subs, ctx, RigidVar(name.into()))
        }
        RigidVar(_) | Structure(_) => {
            // Type mismatch! Rigid can only unify with flex, even if the
            // rigid names are the same.
            merge(subs, ctx, Error(Problem::GenericMismatch))
        }
        Error(problem) => {
            // Error propagates.
            merge(subs, ctx, Error(problem.clone()))
        }
    }
}

#[inline(always)]
fn unify_flex(subs: &mut Subs, ctx: &Context, opt_name: &Option<Box<str>>, other: &Content) {
    match other {
        FlexVar(None) => {
            // If both are flex, and only left has a name, keep the name around.
            merge(subs, ctx, FlexVar(opt_name.clone()))
        }
        FlexVar(Some(_)) | RigidVar(_) | Structure(_) | Error(_) => {
            // In all other cases, if left is flex, defer to right.
            // (This includes using right's name if both are flex and named.)
            merge(subs, ctx, other.clone())
        }
    }
}

fn merge(subs: &mut Subs, ctx: &Context, content: Content) {
    let rank = ctx.first_desc.rank.min(ctx.second_desc.rank);
    let desc = Descriptor {
        content,
        rank,
        mark: Mark::none(),
        copy: None,
    };

    subs.union(ctx.first, ctx.second, desc);
}
