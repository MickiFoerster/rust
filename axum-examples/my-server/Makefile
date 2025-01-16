deploy:
	gcloud services enable cloudbuild.googleapis.com
	gcloud services enable run.googleapis.com 
	gcloud builds submit . 

delete:
	gcloud run services delete my-server --region europe-west3
