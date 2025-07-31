# My strategy to fix the URLs

One of the first issues I encountered after adding two pages to the blog was that I couldn't share one specific page with my friends.
This is because egui builds a single app containing the whole blog, so you need to "configure" the state of the app on startup.

### The already existing solution

The cool thing is that this problem is mostly solved by default in [`eframe_template`](https://github.com/emilk/eframe_template). It shows you how to serialize and deserialize the content of your app to a file.
It wasn't hard to store that in the URL as a query parameter, and it worked for the first month of my blog.
At the time, to be honest, there was no tech blog post but only two recipes I liked that came from a video that I wanted to get written down.
The URL to retrieve the lemon pie recipe was looking like this:
```
https://blog.tamo.cool/?state=%7B"main_article"%3A"Cuisine"%2C"cuisine"%3A%7B"selected"%3A"TarteAuCitron"%7D%7D
```

It works, but it's hard to read and impossible to type.

## Making normal URLs

So, my first thought was that it's not that hard to actually generate and parse a "normal" URL looking like this:
```
https://blog.tamo.cool/cuisine/tarte-au-citron.html
```

After a bit of fiddling, I got it working. No specific tricks were involved here, just a simple match on the URL that set up the correct selected pages.
Then, for every redraw, I compute the current URL before rendering anything and the new URL after everything has been rendered. If something changed, I update the browser's URL so you can just copy the URL when you want to share a page.

## I was happy with this solution but GitHub was not

And that's where the fun begins. That was easy to code and worked well while testing my app with `cargo trunk`.
But once deployed, I lose all control over the router.
My blog is deployed on GitHub Pages, which are made for static websites where every URL matches a file.
That means typing `/` means there must be an `index.html` file at the root.
This one is handled by trunk by default, cool, I don't have anything to do.
For `/cuisine` to work, there must be a file at the same level as `index.html` called `cuisine`, or a directory called `cuisine` that contains an `index.html`.
If there isn't, GitHub returns a `404` and I don't have the opportunity to parse anything.

Sooo, there should be some kind of _file_ loading my application at this location, right...

## Creating tons of files

After a lot of fiddling around, I ended up finding _a_ solution that works well.
I didn't want to make a generic `index.html` that loads my application because that means every time I update the main one, I also have to think of updating the second one used for the links.
And I cannot simply duplicate the main `index.html` either because it contains all the configuration required for building the whole app in a format specific to trunk.
What ended up working well was to create a symlink from the page I want to create to the final generated `index.html`.
```bash
mkdir dist/cuisine
ln -s dist/index.html dist/cuisine/index.html
ln -s dist/index.html dist/cuisine/tarte-au-citron.html
[...]
```

By doing that, I'm sure I'm always pointing to the most updated `index.html` file because there is only one.

## Doing it automatically

Doing that manually in the branch used to build the GitHub page works but is very cumbersome to do by hand.
I could do it with a script, but that means I would have to maintain both my code and a script on the side.
Something I'll absolutely forget to do 9 times out of 10.

What I ended up writing is a small proc-macro that I can call from my Rust code close to the part where I create the enum containing all the pages in this section of my blog.
At the time of writing this blog post, here's what it looks like:
```rust
macros::create_file!(
    "making-this-blog/index.html",
    "making-this-blog/making-the-url-works.html",
    "making-this-blog/formatting-test.html",
);
```

The macro doesn't generate any code, it just generates symlinks at build time in the undocumented `dist/.stage` directory.
A few notes on this trick:
- The symlinks must be relative, otherwise they won't work when deployed in the CI.
- They point to an `index.html` that doesn't exist yet at the time of building because the `index.html` is copied at the very end of the build process.

## Next steps

This macro works well and eases everything. Currently, it's enough for me, but in the future, it could be linked automatically to the enum containing all the variants.
That would also be the occasion to automatically load and export the URL.

When a page contains interactive content, I think it would be nice to be able to represent the state of the content through query parameters.