#include<stdio.h>
#include<ctype.h>
#include<string.h>

int main(){
  int i,j,len;
  char name[200],type,ntype;
  do{
    scanf("%s %c",name,&type);
    if(type=='X') break;
    len=strlen(name);
    if(strchr(name,'_')){
      ntype='D';
    }else if(isupper(name[0])){
      ntype='U';
    }else{
      ntype='L';
    }
    if(type!=ntype){
      name[0]=toupper(name[0]);
      if(ntype=='D'){
	for(i=0;i<len;i++){
	  if(name[i]=='_'){
	    name[i+1]=toupper(name[i+1]);
	    for(j=i+1;name[j];j++) name[j-1]=name[j];
	    name[j-1]=name[j];
	    len--;
	  }
	}
      }
      if(type=='L'){
	name[0]=tolower(name[0]);
      }else if(type=='D'){
	name[0]=tolower(name[0]);
	for(i=1;i<len;i++){
	  if(isupper(name[i])){
	    name[i]=tolower(name[i]);
	    for(j=len;j>i;j--) name[j]=name[j-1];
	    name[j]='_';
	    len++;
	  }
	}
      }
    }
    printf("%s\n",name);
  }while(1);
  return 0;
}