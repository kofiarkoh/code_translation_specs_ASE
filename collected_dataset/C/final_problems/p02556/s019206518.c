#include<stdio.h>
int x[300000],y[300000];
int abs(int k){
	if (k<0) return -k; else return k;
}
int dis(int i,int j){
	return abs(x[i]-x[j])+abs(y[i]-y[j]);
}
void qsx(int l,int r){
	int i,j,temp,mid;
	i=l;j=r;
	mid=(x[i]+x[j])/2;
	while (i<=j){
		while (x[i]<mid) i++;
		while (x[j]>mid) j--;
		if (i<=j){
			temp=x[i];
			x[i]=x[j];
			x[j]=temp;
			temp=y[i];
			y[i]=y[j];
			y[j]=temp;
			i++;j--;
		}
	}
	if (l<j) qsx(l,j);
	if (i<r) qsx(i,r);
}
void qsy(int l,int r){
	int i,j,temp,mid;
	i=l;j=r;
	mid=(y[i]+y[j])/2;
	while (i<=j){
		while (y[i]<mid) i++;
		while (y[j]>mid) j--;
		if (i<=j){
			temp=x[i];
			x[i]=x[j];
			x[j]=temp;
			temp=y[i];
			y[i]=y[j];
			y[j]=temp;
			i++;j--;
		}
	}
	if (l<j) qsy(l,j);
	if (i<r) qsy(i,r);
}
void qsxy(int l,int r){
	int i,j,temp,mid;
	i=l;j=r;
	mid=(x[i]+x[j]+y[i]+y[j])/2;
	while (i<=j){
		while (x[i]+y[i]<mid) i++;
		while (x[j]+y[j]>mid) j--;
		if (i<=j){
			temp=x[i];
			x[i]=x[j];
			x[j]=temp;
			temp=y[i];
			y[i]=y[j];
			y[j]=temp;
			i++;j--;
		}
	}
	if (l<j) qsxy(l,j);
	if (i<r) qsxy(i,r);
}
int main(){
	int n;
	scanf("%d",&n);
	for (int i=1;i<=n;i++){
		scanf("%d %d",&x[i],&y[i]);
	}
	qsx(1,n);
	int ans=0;
	for (int i=1;i<=n;i++){
		if(dis(i,n)>ans)
		  ans=dis(i,n);
	}
	for (int i=1;i<=n;i++){
		if(dis(i,1)>ans)
		  ans=dis(i,1);
	}
	qsy(1,n);
	for (int i=1;i<=n;i++){
		if(dis(i,n)>ans)
		  ans=dis(i,n);
	}
	for (int i=1;i<=n;i++){
		if(dis(i,1)>ans)
		  ans=dis(i,1);
	}
	qsxy(1,n);
	for (int i=1;i<=n;i++){
		if(dis(i,n)>ans)
		  ans=dis(i,n);
	}
	for (int i=1;i<=n;i++){
		if(dis(i,1)>ans)
		  ans=dis(i,1);
	}
	printf("%d",ans);
	
	
	
}