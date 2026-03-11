#include<stdio.h>

#define big(a,b) ((a)<(b)?(a):(b))

int main(){
	int n,i,a[2][50000],ashu[2][100001]={{0}},tmp,ans=0;
	scanf("%d",&n);
	for(i=0;i<n;i++){
		scanf("%d",&tmp);
		ashu[i%2][tmp]++;
	}
	int max[2][2]={{-1,-1},{-1,-1}};
	for(i=0;i<=100000;i++){
		if(max[0][0]<ashu[0][i]){
			max[0][1]=max[0][0];
			max[0][0]=i;
		}else if(max[0][1]<ashu[0][i]){
			max[0][1]=i;
		}
		if(max[1][0]<ashu[1][i]){
			max[1][1]=max[1][0];
			max[1][0]=i;
		}else if(max[1][1]<ashu[1][i]){
			max[1][1]=i;
		}
	}
	if(max[0][0]!=max[1][0])ans=n-ashu[0][max[0][0]]-ashu[1][max[1][0]];
	else {
		ans=big(n-ashu[0][max[0][0]]-ashu[1][max[1][1]],n-ashu[0][max[0][1]]-ashu[1][max[1][0]]);
	}
	printf("%d\n",ans);
	return 0;
}