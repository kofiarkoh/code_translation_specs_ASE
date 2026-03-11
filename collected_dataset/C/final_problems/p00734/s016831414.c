#include <stdio.h>
#include <stdlib.h>
int main(void)
{
	int n,m,i,j;
	int tarou[101];
	int hanako[101];
	int tarou_sum = 0,hanako_sum = 0;
	int sa;
	int min_tarou = 999,min_hanako = 999;
	int min = 999;
	int hen = 0;
	
	while(1){
		scanf("%d %d",&m,&n);
		if(m == 0)break;
		
		min_tarou = 999;
		min_hanako = 999;
		tarou_sum = 0;
		hanako_sum = 0;
		min = 999;
		
		for(i = 0;i < m;i++){
			scanf("%d",&tarou[i]);
			tarou_sum += tarou[i];
		}
		for(i = 0;i < n;i++){
			scanf("%d",&hanako[i]);
			hanako_sum += hanako[i];
		}
		if(!(abs(tarou_sum - hanako_sum) % 2)){
			sa = abs(tarou_sum - hanako_sum) / 2;
			for(i = 0;i < m;i++){
				for(j = 0;j < n;j++){
					if(abs(tarou[i] - hanako[j]) == sa && (tarou[i] + hanako[j]) < min){
						min_tarou = tarou[i];
						min_hanako = hanako[j];
						min = tarou[i] + hanako[j];
					}
				}
				
			}
		}
		
		if(min_tarou == 999){
			printf("-1\n");
		}else{
			printf("%d %d\n",min_tarou,min_hanako);
		}
		
	}
	return 0;
}