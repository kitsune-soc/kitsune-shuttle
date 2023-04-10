# kitsune-shuttle

Shuttle project for the [Kitsune](https://github.com/kitsune-soc/kitsune) social media server

## How do deploy

Update the configuration in `Secrets.toml` to fit your usecase (**definitely** update the domain), pull the submodules, edit the `.gitignore` file in `kitsune/kitsune-fe/.gitignore` and comment out the `dist` directory (since otherwise `cargo-shuttle` will ignore the `dist` directory.  

Now move into `kitsune/kitsune-fe` and build the frontend (`yarn` and `yarn build`), then move back into the main directory and run `cargo shuttle project new` and `cargo shuttle deploy --allow-dirty`.

There you go! Your Shuttle deployment should start up now!

## Configuration

If you want to configure something that isn't possible via the types that are already present inside the example configuration, move into the `kitsune` submodule and copy the `config.example.dhall` file.  
Now you can edit these the file to work for your purposes. Unfortunately you can't just copy the file just like that because it would try to resolve the files referenced in the imports.  
To solve this, you can run `dhall resolve --file [your configuration file]` and copy the resolved configuration file this command outputs into your `Secrets.toml`. This command resolves all the imports.

## License

This project is licensed under the [MIT license](http://opensource.org/licenses/MIT).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, 
shall be licensed as above, without any additional terms or conditions.

