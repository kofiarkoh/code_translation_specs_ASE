#include<stdio.h>
int cmp[100010],r=0,f[10010];
void sdfs(int e,int s,int k,int *ta,int *to,int *nt,int *id){
  int i;//printf("%d\n",s);
  f[s]++;
  for(i=ta[s];i+1;i=nt[i]){
    if(i<e&&k<0&&f[to[i]]<0||i>=e&&k>=0&&!f[to[i]])sdfs(e,to[i],k,ta,to,nt,id);
  }
  k<0?(id[r++]=s):(cmp[s]=k);
}
void scc(int v,int e,int *a,int *b){
  int ta[10010],to[60010],nt[60010],id[10010],i,k;
  for(i=r=0;i<v;i++)f[i]=ta[i]=cmp[i]=-1;
  for(i=0;i<e;i++){
    nt[i  ]=ta[to[i+e]=a[i]];
    nt[i+e]=ta[to[i  ]=b[i]];
    ta[a[i]]=i;
    ta[b[i]]=i+e;
  }
  for(i=k=0;i<v;i++){
    if(f[i]==-1)sdfs(e,i,-1,ta,to,nt,id);
  }
  for(i=r;i;i--){
    if(f[id[i-1]]==0)sdfs(e,id[i-1],id[i-1],ta,to,nt,id);
  }
}
int main(){
  int n,a[10010],b[10010],i,j;
  char s[100010];
  while(scanf("%d",&n),n){
    for(i=0;i<n;i++){
      scanf("%s",s);
      a[i]=s[0]-'a';
      for(j=0;s[j];j++);
      b[i]=s[j-1]-'a';
    }
    scc(26,n,a,b);
    //for(i=0;i<26;i++)printf("%c %d\n",'a'+i,cmp[i]);
    j=cmp[a[0]];
    for(i=0;i<n;i++){
      if(j-cmp[a[i]]||j-cmp[b[i]])break;
    }
    printf("%s\n",i-n?"NG":"OK");
  }
  return 0;
}

