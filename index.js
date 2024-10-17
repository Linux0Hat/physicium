import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');

        let world;
        let world_name;

        function worldCollision() {
            world = wasm.World.new();
            world.add_object(0., -380., 40., -100., 20., 20., 0.8);
            world.add_object(-80., 0., 100., -400., 30., 30., 0.8);
        }
        
        function worldUniversal() {
            world = wasm.World.new();
            world.add_object(0., 0., 0., 0., 50., 82000000000., 0.5);
            world.add_object(0., -300., 250., 0., 20., 10000000., 0.5);
            world.add_object(0., 200., -500., 0., 15., 5000000., 0.5);
            world.set_gravity_y(0.);
            world.set_meter_size(1);
        }

        let world_dictionary = {
            "collision": worldCollision,
            "universal": worldUniversal
        }

        world_name = document.getElementById("world_preset").value;
        world_dictionary[world_name]();

        let view = wasm.WorldView.new(800, 800);

        let t = performance.now();

        function loop() {
            if (document.getElementById("world_preset").value !== world_name) {
                world_name = document.getElementById("world_preset").value;
                world_dictionary[world_name]();
            }

            let elapsed_ms = performance.now() - t;
            t = performance.now();

            if (!document.getElementById("pause").checked) {
                world.apply_physic(elapsed_ms);
            }

            if (document.getElementById("follow").checked) {
                view.set_view_center(world.get_world().objects[0].pos_x, world.get_world().objects[0].pos_y);
            }

            view.draw(world, ctx);
            if (document.getElementById("vectors").checked) {
                view.draw_vectors(world, ctx, 2., document.getElementById("vectors_values").checked);
            }

            requestAnimationFrame(loop);
        }

        // Start the simulation loop
        loop();

    })
    .catch(console.error);
