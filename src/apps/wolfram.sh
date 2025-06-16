#!/bin/bash

read -p "Wolfram App ID [^C to cancel]: " API_KEY
read -p "Enter your question: " QUERY

ENCODED_QUERY=$(echo "$QUERY" | jq -sRr @uri)

RESPONSE=$(curl -s "https://api.wolframalpha.com/v2/query?input=${ENCODED_QUERY}&appid=${API_KEY}&output=json")

RESULT=$(echo "$RESPONSE" | jq -r '.queryresult.pods[]? | select(.title=="Result") | .subpods[0].plaintext')

if [ -z "$RESULT" ]; then
    echo "No result found."
else
    echo "Result: $RESULT"
fi
