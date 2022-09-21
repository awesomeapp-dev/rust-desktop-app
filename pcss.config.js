const prefixer = (await import('autoprefixer')).default;
const nested = (await import('postcss-nested')).default;
const importer = (await import('postcss-import')).default;

const plugins = [
	prefixer,
	importer,
	nested
];


export default {
	// required. Support single string, or array, will be processed in order
	input: ['./src-ui/pcss/main.pcss'],

	// required. single css file supported for now. 
	output: './dist/css/app-bundle.css',

	watchPath: ['./src-ui/pcss/**/*.pcss'],

	// postcss processor arrays
	plugins
}