#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define WLEN 32
#define SLEN 1000
#define W_MAX SLEN/WLEN+1

int main(void){
	int count[W_MAX], p=0, max, maxN, i;
	char *word[W_MAX], *t;
	
	/* init */
	for(i=0; i<W_MAX; i++) count[i]=0;
	
	/* input */
	while(t=malloc(WLEN), ~scanf("%s",t)){
		for(i=0; i<p; i++) if(!strcmp(t,word[i])) break;
		
		count[i]++;
		
		if(i!=p) free(t);
		else word[p++]=t;
	}
	
	/* most frequent word */
	for(max=0, i=0; i<p; i++)
		if(count[i]>max) max=count[i], maxN=i;
	printf("%s ",word[maxN]);
	
	/* longest word */
	for(max=0, i=0; i<p; i++)
		if(strlen(word[i])>max) max=strlen(word[i]), maxN=i;
	printf("%s\n",word[maxN]);
	
	return 0;
}