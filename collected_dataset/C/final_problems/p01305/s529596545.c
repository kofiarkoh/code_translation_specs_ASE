#include <stdio.h>
#include <string.h>

int a[9],b[9],flg[9];
double p,fact9;

void solve(int index,int sumA,int sumB){
	int i;

	if(index == 9 && sumA > sumB){
		p += 1 / fact9;
		return ;
	}

	for(i=0;i<9;i++){
		if(!flg[i]){
			flg[i] = 1;
			if(a[index] > b[i])
				solve(index+1,sumA+a[index]+b[i],sumB);
			else
				solve(index+1,sumA,sumB+a[index]+b[i]);
			flg[i] = 0;
		}
	}
}

int main(void){
	int i,n;

	for(i=2,fact9=1;i<=9;i++) fact9 *= i;

	scanf("%d",&n);

	while(n--){
		for(i=0;i<9;i++) scanf("%d",&a[i]);
		for(i=0;i<9;i++) scanf("%d",&b[i]);

		p = 0;
		memset(flg,0,sizeof(flg));
		solve(0,0,0);

		printf("%.5lf %.5lf\n",p,1-p);
	}

	return 0;
}