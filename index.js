// For more comments about what's going on here, check out the `hello_world`
// example.

import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');

        let world;
        let follow;

        function worldCollision() {
            world = wasm.World.new();
            world.add_object(0., -380., 40., -100., 20., 20., 0.8);
            world.add_object(-80., 0., 100., -400., 30., 30., 0.8);
            follow = false;
        }
        
        function worldUniversal() {
            world = wasm.World.new();
            world.add_object(0., 0., 0., 0., 50., 82000000000., 0.5);
            world.add_object(0., -300., 250., 0., 20., 10000000., 0.5);
            world.add_object(0., 200., -500., 0., 15., 5000000., 0.5);
            world.set_gravity_y(0.);
            world.set_meter_size(1);
            follow = true;
        }

        worldCollision(); // launch collision profile
        worldUniversal(); // launch Universal profile
        
        console.log(world.get_world());
        let view = wasm.WorldView.new(800, 800)
        t=performance.now();
        function loop() {
            let elapsed_ms = performance.now() - t;
            t=performance.now();
            world.apply_physic(elapsed_ms);
            if(follow) {
                view.set_view_center(world.get_world().objects[0].pos_x, world.get_world().objects[0].pos_y);
            } else {view.set_view_center(0., 0.);}
            view.draw(world, ctx);
            requestAnimationFrame(loop);
        }
        loop();
        
    })
  .catch(console.error);
