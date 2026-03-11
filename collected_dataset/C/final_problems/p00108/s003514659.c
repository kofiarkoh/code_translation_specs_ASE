#include<stdio.h>

typedef struct{
  int n;
  int count;
}hoge;
hoge s[13];
int sn;
int main(){
  int seq[13],n,i,j,k,fix;
  do{
    scanf("%d",&n);
    if(n==0) break;
    for(i=0;i<n;i++) scanf("%d ",seq+i);
    for(i=0;;i++){
      for(j=0;j<n;j++)
      fix=1;
      for(j=0;j<n;j++){
	for(k=0;k<sn;k++){
	  if(s[k].n==seq[j]){
	    s[k].count++;
	    break;
	  }
	}
	if(k==sn){
	  s[sn].n=seq[j];
	  s[sn].count=1;
	  sn++;
	}
      }
      for(j=0;j<n;j++){
	for(k=0;k<sn;k++){
	  if(s[k].n==seq[j]){
	    if(seq[j]!=s[k].count) fix=0;
	    seq[j]=s[k].count;
	    break;
	  }
	}
      }
      sn=0;
      if(fix) break;
    }
    printf("%d\n",i);
    for(i=0;i<n-1;i++){
      s[i].n=s[i].count=-1;
      printf("%d ",seq[i]);
    }
    printf("%d\n",seq[i]);
  }while(1);
  return 0;
}