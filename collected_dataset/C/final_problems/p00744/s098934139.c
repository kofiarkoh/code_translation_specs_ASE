#include<stdio.h>
//int ca[500],cb[500],a[500],b[500],n,m;
int g(int t,int r){
  int i=t;
  if(t<r){
    t=r;
    r=i;
  }
  if(r==0)return t;
  return g(r,t%r);
}   
int main(){
  int i,j,min,mi,mj,c;
  int a[500],b[500],n,m;
  while(scanf("%d %d",&n,&m),n||m){
    int d[500][500]={0};
    int ca[500]={0};
    int cb[500]={0};
    for(i=c=0;i<n;i++)scanf("%d",&a[i]);
    for(j=0;j<m;j++)scanf("%d",&b[j]);
    for(i=0;i<n;i++){
      for(j=0;j<m;j++){
	if(g(a[i],b[j])-1){//printf(":%d %d\n",a[i],b[j]);
	  d[i][j]=1;
	  ca[i]++;
	  cb[j]++;
	}//printf("%d ",d[i][j]);
      }//printf("\n");
    }
    //for(i=0;i<n;i++)printf("%d ",ca[i]);printf("\n");
    //for(j=0;j<m;j++)printf("%d ",cb[j]);printf("\n");
    while(1){
      min=2100000000;
      mj=-1;
      for(i=0;i<n;i++){
	if(ca[i]==0)continue;
	if(min>ca[i]){
	  min=ca[i];
	  mi=i;
	}
      }
      for(j=0;j<m;j++){
	if(cb[j]==0)continue;
	if(min>cb[j]){
	  min=cb[j];
	  mj=j;
	}
      }
      if(mj==-1){
	for(j=0;j<m;j++){
	  if(d[mi][j])break;
	}
	mj=j;
      }
      else{
	for(i=0;i<n;i++){
	  if(d[i][mj])break;
	}
	mi=i;
      }
      //printf("%d %d\n",ca[mi],cb[mj]);
      if(min==2100000000)break;
      c++;
      //d[mi][mj]=0;
      for(i=0;i<n;i++){
	if(d[i][mj]){
	  d[i][mj]=0;
	  ca[i]--;
	  cb[mj]--;
	}
      }
      for(j=0;j<m;j++){
	if(mj==j)continue;
	if(d[mi][j]){
	  d[mi][j]=0;
	  ca[mi]--;
	  cb[j]--;
	}
      }
    }
    printf("%d\n",c);
  }
  return 0;
}