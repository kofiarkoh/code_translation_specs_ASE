#include<stdio.h>
int main(){
	int n,i,j,k,a[100],b[100],c[100],d[100],e[100][100],s=0,max=0;
	while(scanf("%d",&n),n){
		for(i=s=0;i<=n;i++)d[i]=0;
		for(i=0;i<n-1;i++){
			scanf("%d %d %d",&a[i],&b[i],&c[i]);
			d[a[i]]++;
			d[b[i]]++;
		}
		d[1]++;
		for(i=0;i<n-1;i++){
			if(d[a[i]]==1||d[b[i]]==1)c[i]=0;
		}
		//for(i=0;i<n-1;i++)printf("%d %d %d\n",a[i],b[i],c[i]);
		for(i=0;i<n-1;i++)s+=c[i];
		for(i=0;i<n*n;i++)e[i/n][i%n]=1e9;
		for(i=0;i<n;i++)e[i][i]=0;
		for(i=0;i<n-1;i++)e[a[i]-1][b[i]-1]=e[b[i]-1][a[i]-1]=c[i];
		for(i=0;i<n;i++){
			for(j=0;j<n;j++){
				for(k=0;k<n;k++){
					if(e[j][k]>e[j][i]+e[i][k])e[j][k]=e[j][i]+e[i][k];
				}
			}
		}
		for(i=0;i<n-1;i++){//printf("%d ",e[0][i]);printf("\n");
			if(max<e[0][i])max=e[0][i];
		}//printf("%d\n",max);
		printf("%d\n",s*2-max);
	}
	return 0;
}