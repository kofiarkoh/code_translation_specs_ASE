#include <stdio.h>
#include <string.h>

typedef struct{
	int pow;
	int flag;
}SOL;

int main(void)
{
	int q,n,i = 0;
	char str[100];
	int a,b,c;
	SOL sol[100000];
	
	scanf("%d %d", &n, &q);
	
	while(q-- > 0){
		scanf("%s %d %d", str, &a, &b);
		
		if(strcmp(str, "IN") == 0){
			scanf("%d", &c);
			if(sol[b].flag == 0 && sol[a].flag == 0) i++;
			
			if(sol[b].flag == 0) {
				sol[b].pow += sol[a].pow;
				sol[b].pow += c;
			}
			else if(sol[a].flag == 0){
				sol[a].pow += sol[b].pow - c;
			}
			sol[b].pow += c;
			sol[a].pow += c;
			sol[a].flag = i;
			sol[b].flag = i;
		}
		else {
			if(sol[a].flag != sol[b].flag){
				printf("WARNING\n");
			}
			
			else if(sol[a].pow != 0 || sol[b].pow != 0){
				printf("%d\n", sol[b].pow - sol[a].pow);
			}
			else {
				printf("WARNING\n");
			}
		}
	}
	
	return(0);
}