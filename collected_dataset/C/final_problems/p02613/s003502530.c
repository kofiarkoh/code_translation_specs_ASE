#include <string.h>
#include <stdio.h>
int main(){
	char w[4][4];
	char s[4];
	int count;
	int r[4];
	int i;
	int j;
	
	strcpy(w[0], "AC");
	strcpy(w[1], "WA");
	strcpy(w[2], "TLE");
	strcpy(w[3], "RE");
	
	r[0] = 0;
	r[1] = 0;
	r[2] = 0;
	r[3] = 0;
	
	scanf("%d", &count);
	for(i = 0; i < count; i++){
		scanf("%s", s);
		for(j = 0; j < 4; j++){
			if(!strcmp(w[j], s)){
				r[j]++;
			}
		}
	}
	
	for(i = 0; i < 4; i++){
		printf("%s x %d\n", w[i], r[i]);
	}
	
	return 0;
}