use crate::{
    geometry::{Rect, SplitAxis},
    Util,
};

fn stack_main_stack(
    window_count: usize,
    container: Rect,
    main_count: usize,
    main_percentage: f32,
    main_split: SplitAxis,
    left_stack_split: SplitAxis,
    right_stack_split: SplitAxis,
    balance_stacks: bool, // true: divide windows evenly between stacks / false: first stack has one window, rest goes to second stack
    reserve_space: bool,
) -> Vec<Rect> {
    let tiles: &mut Vec<Rect> = &mut Vec::new();
    if window_count == 0 {
        return tiles.to_vec();
    }

    let main_window_count = main_count;
    let stack_window_count = window_count.saturating_sub(main_count);

    // the column widths [main/single-stack, stack1, stack2]
    let column_widths: Vec<u32> = match (main_window_count, stack_window_count, reserve_space) {
        (1.., 0, false) | (0, 1.., false) => vec![container.w], // single column
        (1.., 1, false) => {
            // two column
            let main_width = (container.w as f32 / 100.0 * main_percentage) as u32;
            let stack1_width = container.w - main_width;
            vec![main_width, stack1_width]
        }
        (1.., 2.., false) | (_, _, true) => {
            // three column
            let main_width = (container.w as f32 / 100.0 * main_percentage) as u32;
            let remaining_space = (container.w - main_width) as usize;
            let stack_widths = Util::remainderless_division(remaining_space, 2);
            vec![main_width, stack_widths[0] as u32, stack_widths[1] as u32]
        }
        (_, _, _) => vec![],
    };

    let main_tile = match column_widths.len() {
        0 => None,
        1 => Some(container),
        2 => Some(Rect {
            w: column_widths[0],
            x: column_widths[1] as i32,
            ..container
        }),
        _ => {
            Some(Rect {
                w: column_widths[0],
                x: column_widths[2] as i32, // right of stack2
                ..container
            })
        }
    };

    if let Some(tile) = main_tile {
        tiles.append(&mut Util::split(
            &tile,
            main_window_count,
            &main_split,
        ));
    }

    match (main_window_count, stack_window_count) {
        (0, 1..) => {
            let stack_tile = Rect {
                x: container.x + main_tile.map_or(0, |t| t.w) as i32,
                w: container.w - main_tile.map_or(0, |t| t.w),
                ..container
            };
            tiles.append(&mut Util::split(
                &stack_tile,
                stack_window_count,
                &SplitAxis::Horizontal,
            ));
        }
        (1.., 1) => {
            // only one stack window means only one "stack" on the left
            tiles.push(Rect {
                x: 0,
                w: column_widths[1],
                ..container
            });
        }
        (1.., 2..) => {
            let master_tile = main_tile.unwrap();
            let left_stack = Rect {
                x: container.x,
                w: column_widths[2],
                ..container
            };
            let right_stack = Rect {
                x: master_tile.x + master_tile.w as i32,
                w: column_widths[1],
                ..container
            };
            let window_distribution = Util::remainderless_division(stack_window_count, 2);
            tiles.append(&mut Util::split(
                &left_stack,
                window_distribution[0],
                &SplitAxis::Horizontal,
            ));
            tiles.append(&mut Util::split(
                &right_stack,
                window_distribution[1],
                &SplitAxis::Horizontal,
            ));
        }
        (_, _) => {}
    }
    tiles.to_vec()
}