#include <stdio.h>
int main(){

  int n,i,check=0,c;
  char aun[2];

  scanf("%d",&n);
  c=n;

  for(i = 0; i < c; i++){

    if(check == -1){

      printf("NO\n");
      return 0;

    }

    scanf("%s",aun);
    //printf("%s ",aun);                                                                                                                                                                       

    if(aun[0] == 'A') check++;
    else check--;

    // printf("%d ",check);                                                                                                                                                                    

  }

  if(check == 0) printf("YES\n");
  else  printf("NO\n");

  return 0;

}