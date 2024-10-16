// For more comments about what's going on here, check out the `hello_world`
// example.
import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');

        let world = wasm.World.new();
        world.add_object(100., 20., 40., -100., 20., 40., 0.8);
        world.add_object(20., 400., 100., -400., 30., 40., 0.8);

        let view = wasm.WorldView.new(800, 800)
        t=performance.now();
        function loop() {
            let elapsed_ms = performance.now() - t;
            t=performance.now();
            world.apply_physic(elapsed_ms);
            view.draw(world, ctx);
            requestAnimationFrame(loop);
        }
        loop();
        
    })
  .catch(console.error);
