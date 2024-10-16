// For more comments about what's going on here, check out the `hello_world`
// example.
import('./pkg')
    .then(wasm => {
        const canvas = document.getElementById('canvas');
        const ctx = canvas.getContext('2d');

        let world = wasm.World.new();
        world.add_object(100., 20., 0., 0., 20., 40.);
        world.add_object(20., 40., 0., 0., 30., 40.);

        let view = wasm.WorldView.new(800, 800)
        function loop() {
            world.apply_physic(16);
            view.draw(world, ctx);
            requestAnimationFrame(loop);
        }
        loop();
        
    })
  .catch(console.error);
