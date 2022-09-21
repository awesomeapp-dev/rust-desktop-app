// NOTE - Does not need to be ran (svg-symbol.ts already generated). 
//        But if on Mac and Sketch app installed, can generate icons automatically.

export default {
	// change to your app-design.sketch file
	input: 'design/app-design.sketch',
	output: [{
		type: 'svg',
		out: 'src-ui/src/svg-symbols.ts',
		artboard: /^ico\/.*/,
		flatten: '-'
	}]
}