#include<stdio.h>
#define TMAX 100000
int main(void){
	int N, T, time[TMAX]={0}, in, out, max=0, full=0, i, j;
	scanf("%d%d", &N, &T);
	for(i=0; i<N; i++){
		scanf("%d%d", &in, &out);
		if(0 != in || 100000 != out){
			for(j=in; j<out; j++){
				*(time+j) += 1;
				if(max < *(time+j)){
					max = *(time+j);
				}
			}
		}else{
			full++;
		}
	}
	printf("%d\n", max+full);
	return 0;
}