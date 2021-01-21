package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
)

type OptimizelyFile struct {
	Version  string `json:"version"`
	Rollouts []struct {
		ID          string `json:"id"`
		Key         string `json:"key"`
		LayerID     string `json:"layerId"`
		Experiments []struct {
			ID          string `json:"id"`
			Status      string
			AudienceIDs []string `json:"audienceIds"`
		} `json:"experiments"`
		Variations []struct {
			ID             string   `json:"id"`
			Key            string   `json:"key"`
			Variables      []string `json:"variables"`
			FeatureEnabled bool     `json:"featureEnabled"`
		} `json:"variations"`
	} `json:"rollouts"`
	TypedAudiences []interface{} `json:"typedAudiences"`
	Audiences      []struct {
		ID         string `json:"id"`
		Name       string `json:"name"`
		Conditions string `json:"conditions"`
	}
	ProjectID    string   `json:"projectId"`
	Variables    []string `json:"variables"`
	AnonymizeIP  bool     `json:"anonymizeIP"`
	FeatureFlags []struct {
		ExperimentIDs []string `json:"experimentIds"`
		RolloutID     string   `json:"rolloutId"`
		Variables     []struct {
			ID           string      `json:"id"`
			Key          string      `json:"key"`
			Type         string      `json:"type"`
			DefaultValue interface{} `json:"defaultValue"`
		} `json:"variables"`
		ID  string `json:"id"`
		Key string `json:"key"`
	} `json:"featureFlags"`
	Attributes []struct {
		ID  string `json:"id"`
		Key string `json:"key"`
	}
	BotFiltering bool     `json:"botFiltering"`
	AccountID    string   `json:"accountId"`
	Events       []string `json:"events"`
	Revision     string   `json:"revision"`
}

func main() {
	resp, err := http.Get("https://flags.creditkudos.com/Production.json")
	if err != nil {
		log.Fatal(err)
	}
	d, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		log.Fatal(err)
	}
	var f OptimizelyFile
	if err := json.Unmarshal(d, &f); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%+v\n", f)
}
