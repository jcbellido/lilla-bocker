extends Object

## File Position in Package
# As of
# Format of bin entry
#    "PAGE_34_Swedish": {
#      "format": "mp3",
#      "start": 29358559,
#      "length": 10258
#    },

class_name  FilePositionInPackage

var format: String
var start: int
var length: int

# Let's use rust's idiom here
func _init(raw: Dictionary):
	self.format = raw["format"]
	self.start = raw["start"]
	self.length = raw["length"]
