fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    // Fetch the tiles from the model
    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();

    // Duplicate them to ensure that we have pairs
    // tiles.extend(tiles.clone());

    // Randomly mix the tiles
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    // Assign the shuffled Vec to the model property
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();

    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles =
            tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1.image_name == t2.image_name;

            if is_pair_solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);

                let tiles_model = tiles_model.clone();

                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    main_window.run().unwrap();
}

slint::slint! {
    struct TileData {
        image: image,
        image_name: string,
        image_visible: bool,
        solved: bool,
    }

    component MemoryTile inherits Rectangle {
        callback clicked;

        in property <bool> open_curtain;
        in property <bool> solved;
        in property <image> icon;

        height: 128px;
        width: 128px;
        border-width: 2px;
        border-color: black;
        border-radius: 12px;
        clip: true;
        animate background { duration: 800ms; }

        Image {
            source: icon;
            width: parent.width;
            height: parent.height;

            // Solved layer
            Rectangle {
                opacity: 0.3;
                background: solved ? #34CE57 : transparent;
                width: solved ? parent.width : 0;
                height: solved ? parent.height : 0;
            }
        }

        // Left curtain
        Rectangle {
            background: #193076;
            x: 0px;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
        }

        // Right curtain
        Rectangle {
            background: #193076;
            x: open_curtain ? parent.width : (parent.width / 2);
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
            animate x { duration: 250ms; easing: ease-in; }
        }

        TouchArea {
            clicked => {
                // Delegate to the user of this element
                root.clicked();
            }
        }
    }

    export component MainWindow inherits Window {
        width: 562px;
        height: 562px;
        background: #2D2D2D;

        callback check_if_pair_solved(); // Added
        in property <bool> disable_tiles; // Added

        in property <[TileData]> memory_tiles: [
            {
                image: @image-url("icons/boa_noite_img.png"),
                image_name: "boa_noite",
            },
            {
                image: @image-url("icons/boa_noite_text.png"),
                image_name: "boa_noite",
            },
            {
                image: @image-url("icons/boa_tarde_img.png"),
                image_name: "boa_tarde",
            },
            {
                image: @image-url("icons/boa_tarde_text.png"),
                image_name: "boa_tarde",
            },
            {
                image: @image-url("icons/bom_dia_img.png"),
                image_name: "bom_dia",
            },
            {
                image: @image-url("icons/bom_dia_text.png"),
                image_name: "bom_dia",
            },
            {
                image: @image-url("icons/de_nada_img.png"),
                image_name: "de_nada",
            },
            {
                image: @image-url("icons/de_nada_text.png"),
                image_name: "de_nada",
            },
            {
                image: @image-url("icons/obrigado_img.png"),
                image_name: "obrigado",
            },
            {
                image: @image-url("icons/obrigado_text.png"),
                image_name: "obrigado",
            },
            {
                image: @image-url("icons/oi_img.png"),
                image_name: "oi",
            },
            {
                image: @image-url("icons/oi_text.png"),
                image_name: "oi",
            },
            {
                image: @image-url("icons/ola_img.png"),
                image_name: "ola",
            },
            {
                image: @image-url("icons/ola_text.png"),
                image_name: "ola",
            },
            {
                image: @image-url("icons/tudo_bem_img.png"),
                image_name: "tudo_bem",
            },
            {
                image: @image-url("icons/tudo_bem_text.png"),
                image_name: "tudo_bem",
            },
        ];

        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 138px + 10px;
            y: floor(i / 4) * 138px + 10px;
            width: 128px;
            height: 128px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                if (!root.disable_tiles) {
                    tile.image_visible = !tile.image_visible;
                    root.check_if_pair_solved();
                }
            }
        }
    }
}