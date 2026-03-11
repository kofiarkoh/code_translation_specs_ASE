#include <stdio.h>
#include <string.h>
int s;
void delete(int key[2000000],int j){
  int i;
  key[j]=-1;
  for(i=j+1;i>=1;i--){
    if(key[i]!=-1) s=i;
  }
}
int main(void){
  int n,i,j,k,x,key[2000000];
  char com[20];
  k=0;
  scanf("%d",&n);
  for(i=1;i<=n;i++){
    scanf("%s",com);
    if(strcmp(com,"insert")==0){
      scanf("%d",&x);
      k++;
      key[k]=x;
    }
    else if(strcmp(com,"delete")==0){
      scanf("%d",&x);
      for(j=k;j>=1;j--){
	if(key[j]==x){
	  delete(key,j);
	  break;
	}
      }
    }
    else if(strcmp(com,"deleteFirst")==0){
      for(j=k;j>=1;j--){
	if(key[j]!=-1){
	  delete(key,j);
	  break;
	}
      }
    }
    else if(strcmp(com,"deleteLast")==0){
      for(j=1;j<=k;j++){
	if(key[j]!=-1){
	  delete(key,j);
	  break;
	}
      }
    }
  }
  for(i=k;i>=s;i--){
    if(key[i]!=-1){
      printf("%d",key[i]);
      if(i>s) printf(" ");
    }
  }
  printf("\n");
  return 0;
}

