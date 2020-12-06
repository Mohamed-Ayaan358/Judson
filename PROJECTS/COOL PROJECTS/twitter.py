from textblob import TextBlob
import tweepy
import sys

api_key='GnVEdbt7khqLvMRZVtFYjESZE'
api_key_secret='6yfEBej2M33cQmtFVYfCW1rhKguDaBccJed4vmjUQO1VLsYuZZ'
access_token='1330406436409524224-npTMJl5GzOHgd86824OWriUJoUks4c'
access_token_secret='IL0fZXZwzkVirv4b2cBwuNKejt1yZzXq7ZQ7wL1zowid0'

auth_handler= tweepy.OAuthHandler(consumer_key=api_key,consumer_secret=api_key_secret)
auth_handler.set_access_token(access_token,access_token_secret)

api=tweepy.API(auth_handler)

search_term='trump'
tweet_amount=200

tweets=tweepy.Cursor(api.search,q=search_term,lang='en').items(tweet_amount)

polarity=0
positive=0
negative=0
neutral=0
for tweet in tweets:
	final_text=tweet.text.replace('RT','')
	if final_text.startswith(' @'):
		position=final_text.index(':')
		final_text=final_text[position+2:]
	if final_text.startswith('@'):
		position=final_text.index(' ')
		final_text=final_text[position+2:]
	analysis=TextBlob(final_text)
	tweet_polarity=analysis.polarity
	if tweet_polarity>0:
		positive+=1
	elif tweet_polarity<0:
		negative+=1
	else:
		neutral+=1
	polarity+=tweet_polarity

print(polarity)
print(f'Amount of positive tweets : {positive}')
print(f'Amount of negative tweets : {negative}')
print(f'Amount of neutral tweets : {neutral}')



