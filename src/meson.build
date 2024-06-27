pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()
moduledir = pkgdatadir / 'cute_lights'
light_kit_dir = moduledir / 'lib' / 'lightkit'

gnome = import('gnome')
blueprints = custom_target('blueprints',
  input: files(
    'gtk/help-overlay.blp',
    'window.blp',
  ),
  output: '.',
  command: [find_program('blueprint-compiler'), 'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'],
)
gnome.compile_resources('cute_lights',
  'cute_lights.gresource.xml',
  gresource_bundle: true,
  install: true,
  dependencies: blueprints,
  install_dir: pkgdatadir,
)

python = import('python')

conf = configuration_data()
conf.set('PYTHON', python.find_installation('python3').full_path())
conf.set('VERSION', meson.project_version())
conf.set('localedir', get_option('prefix') / get_option('localedir'))
conf.set('pkgdatadir', pkgdatadir)

configure_file(
  input: 'cute_lights.in',
  output: 'cute_lights',
  configuration: conf,
  install: true,
  install_dir: get_option('bindir'),
  install_mode: 'r-xr-xr-x'
)

cute_lights_sources = [
  '__init__.py',
  'main.py',
  'window.py',
  'effect_loader.py',
]

light_kit_sources = [
  'lib/lightkit/__init__.py',
  'lib/lightkit/core.py',
  'lib/lightkit/govee_kit.py',
  'lib/lightkit/hue_kit.py',
  'lib/lightkit/kasa_kit.py',
  'lib/lightkit/openrgb_kit.py',
  'lib/lightkit/utils.py',
  'lib/lightkit/sound.py',
]

install_data(light_kit_sources, install_dir: light_kit_dir)

install_data(cute_lights_sources, install_dir: moduledir)
