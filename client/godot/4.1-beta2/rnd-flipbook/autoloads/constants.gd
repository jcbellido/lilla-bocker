extends Node

const SERVER: String = "http://localhost:8888"

func _ready():
	print("Constants loaded")

func construct_url_of(path: String) -> String:
	return SERVER + "/" + path
