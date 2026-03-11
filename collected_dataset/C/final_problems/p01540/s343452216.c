#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

#define N_MAX 5000

int n,m;
int input_map[N_MAX][2];

int za_xnum;
int za_xlist[N_MAX+1];
int za_xlist_raw[N_MAX+1];
int za_ynum;
int za_ylist[N_MAX+1];
int za_ylist_raw[N_MAX+1];
int za_dupe[N_MAX+1];

int treasure_map[N_MAX+5][N_MAX+5];

/* posList[result]<=pos<posList[result+1] */
int za_get(int pos,int isx) {
	int left,right,mid;
	const int* posList;
	left=0;
	if(isx) {
		right=za_xnum-1;
		posList=za_xlist;
	} else {
		right=za_ynum-1;
		posList=za_ylist;
	}
	while(left<=right) {
		mid=(left+right)/2;
		if(posList[mid]==pos)return mid;
		else if(posList[mid]<pos)left=mid+1;
		else right=mid-1;
	}
	return left-1;
}

int qsort_comp(const void* x,const void* y) {
	int a=*((const int*)x);
	int b=*((const int*)y);
	if(a>b)return 1;
	if(a<b)return -1;
	return 0;
}

void za_setup(void) {
	int i;
	za_xlist_raw[n]=INT_MIN;
	za_ylist_raw[n]=INT_MIN;
	qsort(za_xlist_raw,n+1,sizeof(int),qsort_comp);
	qsort(za_ylist_raw,n+1,sizeof(int),qsort_comp);

	za_dupe[0]=0;
	for(i=1;i<=n;i++) {
		if(za_xlist_raw[i]==za_xlist_raw[i-1])za_dupe[i]=1; else za_dupe[i]=0;
		za_dupe[i]+=za_dupe[i-1];
	}
	for(i=0;i<=n;i++)za_xlist[i-za_dupe[i]]=za_xlist_raw[i];
	za_xnum=n+1-za_dupe[n];

	za_dupe[0]=0;
	for(i=1;i<=n;i++) {
		if(za_ylist_raw[i]==za_ylist_raw[i-1])za_dupe[i]=1; else za_dupe[i]=0;
		za_dupe[i]+=za_dupe[i-1];
	}
	for(i=0;i<=n;i++)za_ylist[i-za_dupe[i]]=za_ylist_raw[i];
	za_ynum=n+1-za_dupe[n];
}

int main(void) {
	int i,j;
	scanf("%d%d",&n,&m);
	for(i=0;i<n;i++) {
		scanf("%d%d",&input_map[i][0],&input_map[i][1]);
		za_xlist_raw[i]=input_map[i][0];
		za_ylist_raw[i]=input_map[i][1];
	}
	za_setup();
	for(i=0;i<n;i++) {
		treasure_map[za_get(input_map[i][0],1)][za_get(input_map[i][1],0)]++;
	}
	for(i=0;i<za_xnum;i++) {
		for(j=1;j<za_ynum;j++) {
			treasure_map[i][j]+=treasure_map[i][j-1];
		}
	}
	for(j=0;j<za_ynum;j++) {
		for(i=1;i<za_xnum;i++) {
			treasure_map[i][j]+=treasure_map[i-1][j];
		}
	}

	for(i=0;i<m;i++) {
		int x1,y1,x2,y2;
		int xx1,yy1,xx2,yy2;
		scanf("%d%d%d%d",&x1,&y1,&x2,&y2);
		xx1=za_get(x1,1);
		yy1=za_get(y1,0);
		xx2=za_get(x2,1);
		yy2=za_get(y2,0);
		if(za_xlist[xx1]==x1)xx1--;
		if(za_ylist[yy1]==y1)yy1--;
		printf("%d\n",
			treasure_map[xx2][yy2]-treasure_map[xx1][yy2]-
			treasure_map[xx2][yy1]+treasure_map[xx1][yy1]);
	}

	return 0;
}