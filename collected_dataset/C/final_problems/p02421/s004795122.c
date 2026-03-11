#include<stdio.h>
#include<string.h>

int main(void)
{
	int n,i,j,k,l;
	int ts,hs;
	char tcard[100],hcard[100];

	scanf("%d",&n);
	ts = 0;
	hs = 0;
	for(i = 0;i < n;i++){
		scanf("%s %s",tcard, hcard);
		if(strcmp(tcard,hcard) == 0){
			ts++;
			hs++;
		}
		else if(strcmp(tcard,hcard) > 0){
			ts += 3;
		}else{
			hs += 3;
		}
	}
	printf("%d %d\n",ts,hs);
	return 0;
}