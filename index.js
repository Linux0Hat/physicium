import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');

        let world;
        let wolrd_preset_actual;

        // Set world to the collision preset
        function worldCollision() {
            world = wasm.World.new();
            world.add_object(0., -380., 40., -100., 20., 20., 0.8, false);
            world.add_object(-80., 0., 100., -400., 30., 30., 0.8, false);
        }
        
        // Set world to the universal gravitation preset
        function worldUniversal() {
            world = wasm.World.new();
            world.add_object(0., 0., 0., 0., 50., 100000000000000000., 0.5, false);
            world.add_object(0., -300., 600., 0., 20., 10000000., 0.5, false);
            world.add_object(0., 200., -700., 0., 15., 5000000., 0.5, false);
            world.set_gravity_y(0.);
            world.set_meter_size(0.001);
        }

        let world_dictionary = {
            "collision": worldCollision,
            "universal": worldUniversal
        }

        wolrd_preset_actual = document.getElementById("world_preset").value;
        world_dictionary[wolrd_preset_actual]();

        let view = wasm.WorldView.new(800, 800);

        document.getElementById("restart_button").onclick = function (){
            world_dictionary[wolrd_preset_actual]();
            view.set_view_center(0., 0.);
        };

        let t = performance.now();

        function loop() {
            // Get the duration of the previous iteration.
            let elapsed_ms = performance.now() - t;
            t = performance.now();

            // Get values from the html file
            var world_preset = document.getElementById("world_preset").value;
            var paused = document.getElementById("pause").checked;
            var follow_enabled = document.getElementById("follow").checked;
            var vector_enabled = document.getElementById("vectors").checked;

            // Change the preset world if the selection changed
            if (world_preset !== wolrd_preset_actual) {
                wolrd_preset_actual = world_preset;
                world_dictionary[wolrd_preset_actual]();
                view.set_view_center(0., 0.);
            }

            // Apply physics.
            if (!paused) {
                world.apply_physic(elapsed_ms);
            }

            // Change the position of the view's center
            if (follow_enabled) {
                view.set_view_center(world.get_world().objects[0].pos_x, world.get_world().objects[0].pos_y);
            }

            // Draw the wolrd in the canvas
            view.draw(world, ctx);
            
            // Draw vectors
            if (vector_enabled) {
                view.draw_vectors(world, ctx, 1., document.getElementById("vectors_values").checked);
            }

            requestAnimationFrame(loop);
        }

        // Start the simulation loop
        loop();

    })
    .catch(console.error);
