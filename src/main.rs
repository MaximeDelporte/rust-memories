use::slint;

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    struct TileData {
        image: image,
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
        background: solved ? #34CE57 : #3960D5;
        animate background { duration: 800ms; }

        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
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
        width: 552px;
        height: 552px;

        in property <[TileData]> memory_tiles: [
            { image: @image-url("icons/boa_noite_img.png") },
            { image: @image-url("icons/boa_noite_text.png") },
            { image: @image-url("icons/boa_tarde_img.png") },
            { image: @image-url("icons/boa_tarde_text.png") },
            { image: @image-url("icons/bom_dia_img.png") },
            { image: @image-url("icons/bom_dia_text.png") },
            { image: @image-url("icons/de_nada_img.png") },
            { image: @image-url("icons/de_nada_text.png") },
            { image: @image-url("icons/obrigado_img.png") },
            { image: @image-url("icons/obrigado_text.png") },
            { image: @image-url("icons/oi_img.png") },
            { image: @image-url("icons/oi_text.png") },
            { image: @image-url("icons/ola_img.png") },
            { image: @image-url("icons/ola_text.png") },
            { image: @image-url("icons/tudo_bem_img.png") },
            { image: @image-url("icons/tudo_bem_text.png") },
        ];

        for tile[i] in memory_tiles : MemoryTile {
            x: mod(i, 4) * 138px;
            y: floor(i / 4) * 138px;
            width: 128px;
            height: 128px;
            icon: tile.image;
            open_curtain: tile.image_visible || tile.solved;
            // propagate the solved status from the model to the tile
            solved: tile.solved;
            clicked => {
                tile.image_visible = !tile.image_visible;
            }
        }
    }
}