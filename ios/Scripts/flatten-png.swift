#!/usr/bin/env swift

// App Store Connect rejects screenshots with an alpha channel, simulator screenshots always have one, and sips cannot remove it from a PNG.
// Usage: swift ios/Scripts/flatten-png.swift <in.png> [out.png]

import CoreGraphics
import Foundation
import ImageIO
import UniformTypeIdentifiers

func fail(_ message: String) -> Never {
	FileHandle.standardError.write(Data("error: \(message)\n".utf8))
	exit(1)
}

let args = Array(CommandLine.arguments.dropFirst())
guard let input = args.first else { fail("usage: flatten-png.swift <in.png> [out.png]") }
let output = args.count > 1 ? args[1] : input
guard let source = CGImageSourceCreateWithURL(URL(fileURLWithPath: input) as CFURL, nil), let image = CGImageSourceCreateImageAtIndex(source, 0, nil) else { fail("could not read \(input)") }
let bounds = CGRect(x: 0, y: 0, width: image.width, height: image.height)
// noneSkipLast, not an opaque alpha channel: the channel itself is what gets rejected.
guard let context = CGContext(data: nil, width: image.width, height: image.height, bitsPerComponent: 8, bytesPerRow: 0, space: CGColorSpace(name: CGColorSpace.sRGB)!, bitmapInfo: CGImageAlphaInfo.noneSkipLast.rawValue) else { fail("could not create a \(image.width)x\(image.height) bitmap") }
context.setFillColor(red: 1, green: 1, blue: 1, alpha: 1)
context.fill(bounds)
context.draw(image, in: bounds)
guard let flattened = context.makeImage(), let destination = CGImageDestinationCreateWithURL(URL(fileURLWithPath: output) as CFURL, UTType.png.identifier as CFString, 1, nil) else { fail("could not flatten \(input)") }
CGImageDestinationAddImage(destination, flattened, nil)
guard CGImageDestinationFinalize(destination) else { fail("could not write \(output)") }
