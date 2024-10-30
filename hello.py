from deepface import DeepFace

def main():
	objs = DeepFace.analyze(
		img_path = "img4.jpg", 
		actions = ['age', 'gender', 'race', 'emotion']
	)


if __name__ == "__main__":
    main()
