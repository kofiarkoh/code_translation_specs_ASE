#include<stdio.h>

int main(){
  int hou,min,sec,tsec;

  while(1){
    scanf("%d %d %d",&hou,&min,&sec);
    if(hou==-1&&min==-1&&sec==-1) break;
    tsec=2*60*60-(hou*60*60+min*60+sec);
    printf("%02d:%02d:%02d\n",tsec/(60*60),(tsec/60)%60,tsec%60);
    tsec=2*60*60-(hou*60*60+min*60+sec)/3;
    printf("%02d:%02d:%02d\n",tsec/(60*60),(tsec/60)%60,tsec%60);
  }
  return 0;
}