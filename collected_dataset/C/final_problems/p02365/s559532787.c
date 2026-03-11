#include<stdio.h>
int ta[10010],to[60010],nt[60010],id[10010],cmp[100010],r=0,f[10010]={0};
int a[10010],b[10010],c[10010],co[10010],q[10010];
void sdfs(int e,int s,int k){
  int i;
  f[s]++;
  for(i=ta[s];i+1;i=nt[i]){
    if(i<e&&k<0&&f[to[i]]==0||i>=e&&k>=0&&f[to[i]]==1)sdfs(e,to[i],k);
  }
  k<0?(id[r++]=s):(cmp[s]=k);
}
void scc(int v,int e,int *a,int *b){
  int i,k;
  for(i=0;i<v;i++)ta[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    ta[a[i]]=i;
    ta[b[i]]=i+e;
  }
  for(i=0;i<v;i++)f[i]=0;
  for(i=k=0;i<v;i++){
    if(f[i]==0)sdfs(e,i,-1);
  }
  for(i=r;i;i--){
    if(f[id[i-1]]==1)sdfs(e,id[i-1],id[i-1]);
  }
}
int mca(int v,int e,int k,int *a,int *b,int *c){
  int ta[110],to[1010],co[1010],nt[1010];
  int i,j,t,r,s,p=1;
  int bk[110]={0};
  int mi[110];
  for(i=0;i<v;i++)ta[i]=mi[i]=-1;//??£??\??????????????????
  for(i=0;i<e;i++){
    nt[i]=ta[a[i]];
    to[i]=b[i];
    co[ta[a[i]]=i]=c[i];
  }
  for(i=0;i<v;i++){
    if(ta[i]+1)p++;
  }
  /*for(i=0;i<v;i++){
    printf("%d:",i);
    for(j=ta[i];j+1;j=nt[j])printf("%d ",to[j]);
    printf("\n");
    }printf("\n");//*/
  for(i=0;i<e;i++){//?????????MIN???????±???????
    if(mi[to[i]]==-1||co[mi[to[i]]]>c[i])mi[to[i]]=i;
  }
  mi[k]=-1;
  /*for(i=0;i<v;i++)if(mi[i]+1)printf("%d %d %d\n",i,mi[i],co[mi[i]]);
  printf("\n");
  for(i=0;i<v;i++){
    printf("%d:",i);
    for(j=ta[i];j+1;j=nt[j]){
      if(mi[to[j]]==j)printf("%d ",to[j]);
    }
    printf("\n");
    }printf("\n");//*/
  q[t=0]=k;
  for(r=1;r-t;t++){
    for(j=ta[q[t]];j+1;j=nt[j]){
      if(mi[to[j]]==j)q[r++]=to[j];
    }
  }
  //printf("n%d r%d\n",p,r);
  if(r==p){
    for(i=s=0;i<e;i++){
      if(mi[to[i]]==i)s+=co[i];
    }
    return s;
  }//printf("d\n");
  for(i=r=0;i<v;i++){
    for(j=ta[i];j+1;j=nt[j]){
      if(mi[to[j]]-j)continue;
      a[r]=i;
      b[r++]=to[j];
    }
  }
  //for(i=0;i<r;i++)printf("%d %d\n",a[i],b[i]);printf("\n");
  scc(v,r,a,b);//?????£?????????????§£??§???????????????????????????
  //for(i=0;i<v;i++)printf("%d %d\n",i,cmp[i]);printf("\n");
  //printf("f\n");
  for(i=0;i<v;i++)bk[cmp[i]]++;
  for(i=r=s=0;i<v;i++){//printf("%d %d:",i,cmp[i]);
    for(j=ta[i];j+1;j=nt[j]){//printf("%d %d  ",to[j],cmp[to[j]]);
      a[r]=cmp[i];
      b[r]=cmp[to[j]];
      //if(bk[a[r]]-1){
	if(a[r]==b[r]){
	  if(j==mi[to[j]])s+=co[j];
	}
	//else c[r++]=co[j];
	//}
      else c[r++]=bk[b[r]]-1?co[j]-co[mi[to[j]]]:co[j];
    }//printf("\n");
  }
  //printf("s%d r%d\n",s,r);
  //for(i=0;i<r;i++)printf("%d %d %d\n",a[i],b[i],c[i]);printf("\n");
  return mca(v,r,k,a,b,c)+s;
}
	 
int main(){
  int n,m,k,i;
  scanf("%d %d %d",&n,&m,&k);
  for(i=0;i<m;i++)scanf("%d %d %d",&a[i],&b[i],&c[i]);
  printf("%d\n",mca(n,m,k,a,b,c));
  return 0;
}