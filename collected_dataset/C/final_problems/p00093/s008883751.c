#include <stdio.h>

int main(void){
	int a,b,i;
	int isFirst = 1;
	int flag;
	while(1){
		scanf("%d%d",&a,&b);
		if(a == 0 && b == 0){break;}
		if(!isFirst){putchar('\n');}
		isFirst = 0;
		flag = 0;
		for(i = a; i <= b; i++){
			if(!(i % 4)){
				if(!(i % 100)){
					if(!(i % 400)){
						printf("%d\n",i);
						flag = 1;
					}
				}else{
					printf("%d\n",i);
					flag = 1;
				}
			}
		}
		if(!flag){puts("NA");}
	}

	return 0;
}