#include <stdio.h>
#include <stdlib.h>

int main()
{

	int N;
	int i;
	int *p;
	int x=0;
	int c=0;

	scanf("%d", &N);

	p = (int*)malloc(sizeof(int) * N);
	if (p == NULL){
		printf("メモリが確保できません。\n");
		return 0;
	}

	for(i=0; i<N; i++){
		scanf("%d", p+i);
	}

	while(1){
		c++;
		if(*(p+x) == 2){
			break;
		}
		if(c == N+1){
			break;
		}
		x = *(p+x) - 1;
	}

	if(c == N+1){
		printf("-1\n");
	}
	else{
		printf("%d", c);
	}

return 0;
}