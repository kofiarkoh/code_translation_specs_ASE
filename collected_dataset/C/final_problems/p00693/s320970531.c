#include<stdio.h>
#include<stdlib.h>
#include<string.h>

typedef struct Rule{
  char legal[7],sou[9],des[9];
}rule;

typedef struct Packet{
  char sou[9],des[9],mes[51];
}packet;


int main(void){
  int i,j,k;
  int n,m;
  rule r[1024];
  packet p[1024];
  int ans[1024],count;
  int flag;

  while(1){
    scanf(" %d %d ",&n,&m);
    if(!n && !m)break;

    for(i=0;i<n;i++){
      fscanf(stdin," %s ",r[i].legal);
      fscanf(stdin," %s ",r[i].sou);
      fscanf(stdin," %s ",r[i].des);
    }
    for(i=0;i<m;i++){
      fscanf(stdin," %s ",p[i].sou);
      fscanf(stdin," %s ",p[i].des);
      fscanf(stdin," %s ",p[i].mes);
    }
    
    count = 0;
    for(i=0;i<m;i++){
      for(j=n-1;j>=0;j--){
	flag = 0;
	for(k=0;k<8;k++){
	  if(r[j].sou[k] != '?' && r[j].sou[k] != p[i].sou[k]){
	      flag = 1;
	      break;
	    }
	}
	if(!flag){
	  for(k=0;k<8;k++){
	    if(r[j].des[k] != '?' && r[j].des[k] != p[i].des[k]){
	      flag = 1;
	      break;
	    }
	  }
	}
	if(!flag){
	  if(!strcmp(r[j].legal,"permit")){
	    ans[count] = i;
	    count++;
	    break;
	  }else{
	    break;
	  }
	}
      }
    }
      printf("%d\n",count);
      for(i=0;i<count;i++){
	printf("%s ",p[ans[i]].sou);
	printf("%s ",p[ans[i]].des);
	printf("%s\n",p[ans[i]].mes);
      }

  }
  return 0;
}