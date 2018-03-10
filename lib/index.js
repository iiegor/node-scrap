const path = require('path');
const assert = require('assert');
const addon = require('../native');

/** Record the screen */
const opts = {
  fps: "30",
  path: path.join(__dirname, '..', 'video.mp4'),
};

addon.startRecording(opts.path, opts.fps);

/** Take screenshot */
// addon.screenshot(path.join(__dirname, '..', 'screenshot.png'));

module.exports = addon;
