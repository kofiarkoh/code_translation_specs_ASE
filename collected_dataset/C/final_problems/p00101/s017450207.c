#include <stdio.h>
#include <string.h>

int main(void) {
	char eibun[1001];
	char* pos;
	int kazu,nowkazu;
	scanf("%d",&kazu);
	while(getchar()!='\n');
	for(nowkazu=0;nowkazu<kazu;nowkazu++) {
		fgets(eibun,sizeof(eibun),stdin);
		while(pos=strstr(eibun,"Hoshino")) {
			*(pos+6)='a';
		}
		printf("%s",eibun);
	}
	return 0;
}