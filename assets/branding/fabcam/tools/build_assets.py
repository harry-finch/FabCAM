"""Build review assets; does not change any application configuration.

Requires Pillow, Inkscape and Inter Medium/Bold visible to fontconfig.
Use FONTCONFIG_FILE for a temporary font environment; no font files are bundled.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile
import xml.etree.ElementTree as ET
from PIL import Image, ImageChops, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
INKSCAPE = os.environ.get('INKSCAPE', shutil.which('inkscape') or '/Applications/Inkscape.app/Contents/MacOS/inkscape')
SIZES = [16, 20, 24, 32, 48, 64, 128, 256]
APP_SIZES = SIZES + [512, 1024]
NS = 'http://www.w3.org/2000/svg'
ET.register_namespace('', NS)

def run(*args, env=None):
    return subprocess.run(args, check=True, capture_output=True, text=True, env=env).stdout

def render(svg, png, size=None, env=None):
    args = [INKSCAPE, str(svg), '--export-type=png', f'--export-filename={png}']
    if size:
        args += [f'--export-width={size}']
    run(*args, env=env)

def mono(source, target, color):
    tree = ET.parse(source)
    root = tree.getroot()
    for el in root.iter():
        for key in ('fill', 'stroke'):
            if el.get(key, '').startswith('#'):
                el.set(key, color)
        if 'style' in el.attrib:
            import re
            el.set('style', re.sub(r'(fill|stroke):#[0-9a-fA-F]{3,8}', lambda m: m.group(1) + ':' + color, el.get('style')))
    for el in root.iter(f'{{{NS}}}title'):
        el.text = 'FabCAM A1 monochrome ' + ('light' if color == '#EEF3F8' else 'dark')
    root.set('aria-label', 'FabCAM A1 monochrome logo')
    tree.write(target, encoding='unicode')

def build():
    for folder in ['variants', 'exports/png', 'exports/windows', 'exports/macos', 'exports/linux', 'previews']:
        (ROOT / folder).mkdir(parents=True, exist_ok=True)
    source = ROOT / 'source'
    png = ROOT / 'exports/png'
    variants = ROOT / 'variants'
    # Use the declared first font family, never an unnoticed fallback.
    for font in ['Inter:weight=medium', 'Inter:weight=bold']:
        resolved = run('fc-match', '-f', '%{family}', font)
        if resolved.strip().split(',')[0] != 'Inter':
            raise RuntimeError(f'{font} resolves to {resolved}; expose genuine Inter first')
    outlined = source / 'fabcam-a1-lockup-outlined.svg'
    run(INKSCAPE, str(source / 'fabcam-a1-lockup.svg'), '--export-type=svg', '--export-text-to-path', '--export-plain-svg', f'--export-filename={outlined}')
    tree = ET.parse(outlined)
    root = tree.getroot()
    for el in list(root):
        if el.tag.split('}')[-1] in ('metadata', 'namedview') or (el.tag.split('}')[-1] == 'defs' and not list(el)):
            root.remove(el)
    for el in root.iter():
        el.attrib.pop('id', None)
        el.attrib.pop('version', None)
        for key in list(el.attrib):
            if key.startswith('{') and not key.startswith('{' + NS + '}'):
                del el.attrib[key]
        if 'style' in el.attrib:
            # Typography properties have no effect on outlines.
            entries = el.get('style').split(';')
            el.set('style', ';'.join(e for e in entries if e and not e.startswith(('font', '-inkscape', 'letter-spacing', 'line-height', 'text-align', 'text-anchor'))))
    tree.write(outlined, encoding='unicode')
    assert not list(root.iter(f'{{{NS}}}text'))
    micro = variants / 'fabcam-symbol-micro.svg'
    data = (source / 'fabcam-a1-symbol.svg').read_text()
    data = data.replace('M20 28.6 V25', 'M20 26.6 V25').replace('Q20 44 20 39', 'Q20 44 20 41').replace('Optical Balance symbol', 'Optical Balance micro candidate symbol')
    micro.write_text(data)
    for name, color in [('light', '#EEF3F8'), ('dark', '#171E28')]:
        mono(source / 'fabcam-a1-symbol.svg', variants / f'fabcam-symbol-{name}.svg', color)
        mono(outlined, variants / f'fabcam-lockup-{name}.svg', color)
    for size in APP_SIZES:
        render(source / 'fabcam-a1-app-icon.svg', png / f'fabcam-app-icon-{size}.png', size)
    for size in SIZES:
        render(source / 'fabcam-a1-symbol.svg', png / f'fabcam-symbol-{size}.png', size)
    for size in [16, 20, 24]:
        render(micro, png / f'fabcam-symbol-micro-{size}.png', size)
    render(outlined, png / 'fabcam-lockup.png')
    # ICO embeds actual multi-resolution images, including a 256 px PNG payload.
    im = Image.open(png / 'fabcam-app-icon-256.png')
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    images = [Image.open(png / f'fabcam-app-icon-{s}.png') for s in ico_sizes]
    im.save(ROOT / 'exports/windows/fabcam.ico', sizes=[(s, s) for s in ico_sizes], append_images=images)
    # Use iconutil's standard 1x/2x slots, with matching directly rendered PNGs.
    if shutil.which('iconutil'):
        with tempfile.TemporaryDirectory(prefix='fabcam-a1-') as temp:
            iconset = Path(temp) / 'FabCAM.iconset'
            iconset.mkdir()
            for size in [16, 32, 128, 256, 512]:
                for scale in [1, 2]:
                    suffix = '@2x' if scale == 2 else ''
                    shutil.copyfile(png / f'fabcam-app-icon-{size*scale}.png', iconset / f'icon_{size}x{size}{suffix}.png')
            run('iconutil', '-c', 'icns', str(iconset), '-o', str(ROOT / 'exports/macos/fabcam.icns'))
        # iconutil's legacy is32/il32 chunks alter RGB on partially transparent
        # edges. Replace those slots with supported lossless PNG icp4/icp5
        # entries; retain every native-generated Retina/larger chunk.
        icns = ROOT / 'exports/macos/fabcam.icns'
        data = icns.read_bytes()
        chunks, cursor = [], 8
        while cursor < len(data):
            kind = data[cursor:cursor+4]
            length = struct.unpack('>I',data[cursor+4:cursor+8])[0]
            if kind not in [b'is32',b's8mk',b'il32',b'l8mk',b'TOC ']:
                chunks.append(data[cursor:cursor+length])
            cursor += length
        for kind,size in [(b'icp4',16),(b'icp5',32)]:
            payload = (png / f'fabcam-app-icon-{size}.png').read_bytes()
            chunks.append(kind + struct.pack('>I',len(payload)+8) + payload)
        body = b''.join(chunks)
        icns.write_bytes(b'icns' + struct.pack('>I',len(body)+8) + body)
    for size in APP_SIZES:
        link = ROOT / f'exports/linux/fabcam-{size}.png'
        if not link.exists():
            link.symlink_to(f'../png/fabcam-app-icon-{size}.png')
    review()
    validate()

def review():
    bg, dark, white = '#E6E9ED', '#202731', '#FAFAFB'
    sheet = Image.new('RGB', (1120, 1090), bg)
    d = ImageDraw.Draw(sheet)
    title = ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf', 22)
    body = ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf', 15)
    def label(x, y, text, color='#29313B', font=body):
        d.text((x,y), text, fill=color, font=font)
    def paste(path, x, y):
        image = Image.open(path).convert('RGBA')
        sheet.paste(image, (x,y), image)
    label(28, 22, 'FabCAM · A1 Optical Balance — production review', font=title)
    label(28, 55, 'Approved geometry and palette preserved · micro variant is a separate candidate')
    d.rectangle((24, 90, 1096, 310), fill=dark)
    label(40, 105, 'Canonical A1', '#EEF3F8')
    paste(ROOT / 'exports/png/fabcam-symbol-128.png', 44, 146)
    label(234, 105, 'App tile', '#EEF3F8')
    paste(ROOT / 'exports/png/fabcam-app-icon-128.png', 242, 146)
    label(450, 105, 'Inter Medium 500 / Bold 700 · outlined', '#EEF3F8')
    paste(ROOT / 'exports/png/fabcam-lockup.png', 450, 170)
    label(28, 332, 'Monochrome · one foreground color per asset', font=title)
    for name, x, background in [('light',24,dark), ('dark',572,white)]:
        d.rectangle((x, 375, x+524, 525), fill=background)
        with tempfile.TemporaryDirectory() as temp:
            sym, lock = Path(temp)/'symbol.png', Path(temp)/'lock.png'
            render(ROOT / f'variants/fabcam-symbol-{name}.svg', sym, 96)
            render(ROOT / f'variants/fabcam-lockup-{name}.svg', lock)
            paste(sym,x+12,398)
            paste(lock,x+120,405)
    label(28, 547, 'Actual-size rasters · view at 100% / 1 image pixel per display pixel', font=title)
    cols = [155,240,325,425,565,750,920]
    sizes = [16,20,24,32,48,64,128]
    for x,size in zip(cols,sizes):
        label(x,583,f'{size}px')
    for row, name in enumerate(['Canonical','Micro candidate','App tile']):
        y = 615 + row*150
        d.rectangle((24,y,1096,y+140),fill=dark)
        label(35,y+12,name,'#EEF3F8')
        for x,size in zip(cols,sizes):
            if row == 1 and size > 24:
                continue
            base = 'fabcam-symbol' if row == 0 else ('fabcam-symbol-micro' if row == 1 else 'fabcam-app-icon')
            paste(ROOT / f'exports/png/{base}-{size}.png',x,y+6+(128-size)//2)
    label(28,1070,'Micro: inner opening only; compare 16–20 px. Canonical recommended from 24 px.')
    sheet.save(ROOT / 'previews/logo-review.png')
    # Full eight-size strip, including the requested 256 px inspection.
    sheet = Image.new('RGB',(1024,300),dark)
    d = ImageDraw.Draw(sheet)
    x = 20
    for size in SIZES:
        d.text((x,12),f'{size}px',fill='white',font=body)
        im = Image.open(ROOT / f'exports/png/fabcam-symbol-{size}.png')
        sheet.paste(im,(x,40),im)
        x += max(70,size+20)
    sheet.save(ROOT / 'previews/symbol-size-review.png')
    sheet = Image.new('RGB',(800,500),bg)
    d = ImageDraw.Draw(sheet)
    label(24,18,'Canonical A1 / micro candidate · side by side',font=title)
    label(24,52,'Actual size beside nearest-neighbor 4× pixel inspection')
    for column,name in enumerate(['Canonical A1','Micro candidate']):
        x = 24 + column*390
        d.rectangle((x,90,x+365,480),fill=dark)
        label(x+12,100,name,'#EEF3F8')
        for row,size in enumerate([16,20,24]):
            y = 138 + row*110
            label(x+12,y+4,f'{size}px','#EEF3F8')
            stem = 'fabcam-symbol' + ('-micro' if column else '')
            im = Image.open(ROOT/f'exports/png/{stem}-{size}.png')
            sheet.paste(im,(x+70,y+24),im)
            # Fit each row into a fixed 96 px viewport; magnification is 4×.
            big = im.resize((size*4,size*4),Image.Resampling.NEAREST)
            sheet.paste(big,(x+170,y),big)
    sheet.save(ROOT/'previews/micro-comparison.png')

def validate():
    report = {'svg':{},'png':{},'ico':{},'icns':{},'font':{'family':'Inter','release':'4.1','weights':[500,700],'bundled':False}}
    def geometric_children(file):
        return [(el.tag.split('}')[-1], {k:v for k,v in el.attrib.items() if k != 'id'}) for el in ET.parse(file).getroot().iter() if el.tag.split('}')[-1] in ['rect','path']]
    geometry = geometric_children(ROOT/'source/fabcam-a1-symbol.svg')
    assert geometric_children(ROOT/'source/fabcam-a1-app-icon.svg')[1:] == geometry
    assert geometric_children(ROOT/'source/fabcam-a1-lockup.svg') == geometry
    assert geometric_children(ROOT/'source/fabcam-a1-lockup-outlined.svg')[:3] == geometry
    report['canonical_geometry'] = {'symbol_app_lockup_consistent':True,'outlined_symbol_unchanged':True,'micro_change':'inner path start y 28.6 → 26.6; end y 39 → 41; all other geometry identical'}
    for file in list((ROOT/'source').glob('*.svg')) + list((ROOT/'variants').glob('*.svg')):
        root = ET.parse(file).getroot()
        import re
        colors = set(c.upper() for c in re.findall(r'#[0-9a-fA-F]{6}',file.read_text()))
        allowed = {'#58B5F5','#171E28','#354250','#EEF3F8','#75C8FA'}
        assert colors <= allowed
        if file.parent.name == 'variants' and 'micro' not in file.name:
            assert len(colors) == 1
        for el in root.iter():
            assert el.tag.split('}')[-1] not in ['image','script','foreignObject']
            assert not any(k.endswith('href') for k in el.attrib)
            assert 'url(' not in el.get('style','')
        with tempfile.TemporaryDirectory() as temp:
            raster = Path(temp)/'check.png'
            render(file,raster)
            im = Image.open(raster).convert('RGBA')
            assert im.getpixel((0,0))[3] == 0
            bounds = im.getchannel('A').getbbox()
            assert bounds[0] > 0 and bounds[1] > 0 and bounds[2] < im.width and bounds[3] < im.height
        report['svg'][str(file.relative_to(ROOT))] = {'parse':True,'render':True,'transparent_corner':True,'bounds':bounds,'colors':sorted(colors),'sha256':hashlib.sha256(file.read_bytes()).hexdigest()}
    for file in (ROOT/'exports/png').glob('*.png'):
        im = Image.open(file)
        im.verify()
        im = Image.open(file)
        report['png'][file.name] = list(im.size)
        if file.stem.split('-')[-1].isdigit():
            size = int(file.stem.split('-')[-1])
            assert im.size == (size,size)
    ico = Image.open(ROOT/'exports/windows/fabcam.ico')
    assert ico.format == 'ICO'
    sizes = sorted(ico.ico.sizes())
    assert sizes == [(s,s) for s in [16,24,32,48,64,128,256]]
    for size in sizes:
        decoded = ico.ico.getimage(size).convert('RGBA')
        assert decoded.size == size
        original = Image.open(ROOT/f'exports/png/fabcam-app-icon-{size[0]}.png').convert('RGBA')
        assert decoded.tobytes() == original.tobytes()
    report['ico'] = {'sizes':sizes,'decoded_pixels_match_source_pngs':True}
    icns = ROOT/'exports/macos/fabcam.icns'
    if icns.exists():
        data = icns.read_bytes()
        assert data[:4] == b'icns' and struct.unpack('>I',data[4:8])[0] == len(data)
        im = Image.open(icns)
        assert im.format == 'ICNS'
        encoded_sizes = im.info['sizes']
        for size in encoded_sizes:
            decoded = Image.open(icns).icns.getimage(size).convert('RGBA')
            original = Image.open(ROOT/f'exports/png/fabcam-app-icon-{decoded.width}.png').convert('RGBA')
            assert decoded.tobytes() == original.tobytes()
        im.load()
        with tempfile.TemporaryDirectory() as temp:
            dest = Path(temp)/'roundtrip.iconset'
            run('iconutil','-c','iconset',str(icns),'-o',str(dest))
            extracted, differences = {}, {}
            for file in dest.glob('*.png'):
                result = Image.open(file)
                result.verify()
                result = Image.open(file).convert('RGBA')
                extracted[file.name] = list(result.size)
                original = Image.open(ROOT/f'exports/png/fabcam-app-icon-{result.width}.png').convert('RGBA')
                assert result.getchannel('A').tobytes() == original.getchannel('A').tobytes()
                white = Image.new('RGBA',result.size,'white')
                visible = ImageChops.difference(Image.alpha_composite(white,result),Image.alpha_composite(white,original))
                differences[file.name] = {'identical_rgba':result.tobytes()==original.tobytes(),'visible_channel_extrema_on_white':visible.getextrema(),'alpha_identical':True}
        assert len(extracted) == 10
        report['icns'] = {'header':True,'encoded_png_pixels_match_source_pngs':True,'iconutil_roundtrip':extracted,'native_extraction_differences':differences}
    # Conversion fidelity: render live text and outlines using the same font environment.
    with tempfile.TemporaryDirectory() as temp:
        a,b = Path(temp)/'live.png',Path(temp)/'outline.png'
        render(ROOT/'source/fabcam-a1-lockup.svg',a,1288)
        render(ROOT/'source/fabcam-a1-lockup-outlined.svg',b,1288)
        a,b = Image.open(a).convert('RGBA'),Image.open(b).convert('RGBA')
        diff = ImageChops.difference(a,b)
        pixels = list(diff.get_flattened_data())
        changed = sum(max(p)>0 for p in pixels)
        report['wordmark_conversion'] = {'raster_dimensions':list(a.size),'difference_bounds':diff.getbbox(),'identical_pixels':a.tobytes()==b.tobytes(),'changed_pixels':changed,'total_pixels':len(pixels),'max_channel_difference':max(max(p) for p in pixels),'mean_absolute_channel_difference':sum(sum(p) for p in pixels)/(len(pixels)*4),'live_alpha_bounds':a.getchannel('A').getbbox(),'outline_alpha_bounds':b.getchannel('A').getbbox()}
        default_env = dict(os.environ)
        default_env.pop('FONTCONFIG_FILE',None)
        nofont = Path(temp)/'default-fontconfig.png'
        render(ROOT/'source/fabcam-a1-lockup-outlined.svg',nofont,1288,env=default_env)
        assert Image.open(nofont).convert('RGBA').tobytes() == b.tobytes()
        report['wordmark_conversion']['outlined_render_identical_without_custom_fontconfig'] = True
    assert not list(ROOT.rglob('*.ttf')) and not list(ROOT.rglob('*.otf')) and not list(ROOT.rglob('*.woff*'))
    (ROOT/'previews/validation.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({'SVGs':len(report['svg']),'PNGs':len(report['png']),'ICO':sizes,'ICNS':bool(report['icns']),'wordmark':report['wordmark_conversion']}))

def integrate():
    """Populate established application paths from approved masters/exports.

    No Tauri identity/config changes; no font is needed for this command.
    Android is deliberately outside this desktop integration checkpoint.
    """
    repo = ROOT.parents[2]
    public = repo/'frontend/public'
    branding = public/'branding'
    branding.mkdir(exist_ok=True)
    png = ROOT/'exports/png'
    icons = repo/'crates/ivac-tauri/icons'
    mappings = {
        ROOT/'source/fabcam-a1-symbol.svg': [branding/'fabcam-symbol.svg',public/'favicon.svg'],
        ROOT/'source/fabcam-a1-lockup-outlined.svg': [branding/'fabcam-lockup.svg'],
        ROOT/'source/fabcam-a1-app-icon.svg': [icons/'icon.svg'],
        ROOT/'exports/windows/fabcam.ico': [icons/'icon.ico'],
        ROOT/'exports/macos/fabcam.icns': [icons/'icon.icns'],
    }
    for size in [16,32,64,128,256]:
        mappings[png/f'fabcam-symbol-{size}.png'] = [public/f'favicon-{size}.png']
    for name,size in [('32x32.png',32),('64x64.png',64),('128x128.png',128),('128x128@2x.png',256),('icon.png',512)]:
        mappings[png/f'fabcam-app-icon-{size}.png'] = [icons/name]
    for source,targets in mappings.items():
        for target in targets:
            shutil.copyfile(source,target)
    render(ROOT/'source/fabcam-a1-app-icon.svg',public/'apple-touch-icon.png',180)
    for size in [30,44,71,89,107,142,150,284,310]:
        render(ROOT/'source/fabcam-a1-app-icon.svg',icons/f'Square{size}x{size}Logo.png',size)
    render(ROOT/'source/fabcam-a1-app-icon.svg',icons/'StoreLogo.png',50)
    conf = json.loads((repo/'crates/ivac-tauri/tauri.conf.json').read_text())
    for path in conf['bundle']['icon']:
        assert (repo/'crates/ivac-tauri'/path).is_file()
    for folder in [public,icons]:
        for file in folder.glob('*.png'):
            if file.name.startswith(('favicon-','Square')) or file.name in ['32x32.png','64x64.png','128x128.png','128x128@2x.png','icon.png','StoreLogo.png','apple-touch-icon.png']:
                im = Image.open(file)
                im.verify()
                assert Image.open(file).convert('RGBA').getpixel((0,0))[3] == 0
    expected = {public/f'favicon-{s}.png':s for s in [16,32,64,128,256]}
    expected[public/'apple-touch-icon.png'] = 180
    expected.update({icons/f'Square{s}x{s}Logo.png':s for s in [30,44,71,89,107,142,150,284,310]})
    expected.update({icons/name:size for name,size in [('32x32.png',32),('64x64.png',64),('128x128.png',128),('128x128@2x.png',256),('icon.png',512),('StoreLogo.png',50)]})
    dimensions = {}
    for file,size in expected.items():
        actual = Image.open(file).size
        assert actual == (size,size)
        dimensions[str(file.relative_to(repo))] = list(actual)
    report = {'policy':'canonical only; micro candidate excluded','raster_dimensions':dimensions,'tauri_icon_paths':conf['bundle']['icon'],'tauri_identity':{key:conf[key] for key in ['productName','identifier','version']},'svg_sources':{str(target.relative_to(repo)):hashlib.sha256(source.read_bytes()).hexdigest() for source,targets in mappings.items() if source.suffix=='.svg' for target in targets},'native_exports_byte_identical':all((icons/name).read_bytes()==(ROOT/path).read_bytes() for name,path in [('icon.ico','exports/windows/fabcam.ico'),('icon.icns','exports/macos/fabcam.icns')])}
    (ROOT/'previews/integration-validation.json').write_text(json.dumps(report,indent=2)+'\n')
    sheet = Image.new('RGB',(900,335),'#202731')
    draw = ImageDraw.Draw(sheet)
    font = ImageFont.truetype('/System/Library/Fonts/Supplemental/Arial.ttf',15)
    draw.text((20,14),'FabCAM A1 · canonical app tile · actual-size PNG exports',fill='#EEF3F8',font=font)
    x = 20
    for size in [16,24,32,64,128,256]:
        draw.text((x,46),f'{size}px',fill='#EEF3F8',font=font)
        im = Image.open(png/f'fabcam-app-icon-{size}.png')
        sheet.paste(im,(x,70),im)
        x += max(80,size+30)
    sheet.save(ROOT/'previews/app-icon-size-review.png')
    print('Canonical A1 integrated into existing frontend/Tauri icon paths; config unchanged.')

if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--integrate',action='store_true',help='Copy approved assets and render desktop/browser sizes; no font required')
    args = parser.parse_args()
    if args.integrate:
        integrate()
    else:
        build()
