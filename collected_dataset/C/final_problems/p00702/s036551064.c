//suhan lee, saitama university;
#define ASK 97
#include <stdio.h>
#include <string.h>
// kanglish[38][3]={a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,ld,mb,mp,nc,nd,ng,nt,nw,ps,qu,cw,ts};


int main(){
  int i,k,fl2,j,num,tem,flag;
  char head,tail,inp;
  char kanglish[38][3];
  char ans[38];
  char field[40][40];
  for(i=0;i<39;i++)
    for(j=0;j<39;j++)
      field[i][j]=0;
  for(i=0;i<26;i++)
    {
      kanglish[i][0]=ASK+i;
      kanglish[i][1]=0;

    }

  strcpy(kanglish[26],"ld");
  strcpy(kanglish[27],"mb"); 
  strcpy(kanglish[28],"mp"); 
  strcpy(kanglish[29],"nc"); 
  strcpy(kanglish[30],"nd"); 
  strcpy(kanglish[31],"ng"); 
  strcpy(kanglish[32],"nt"); 
  strcpy(kanglish[33],"nw"); 
  strcpy(kanglish[34],"ps"); 
  strcpy(kanglish[35],"qu"); 
  strcpy(kanglish[36],"cw"); 
  strcpy(kanglish[37],"ts"); 


 
  scanf("%d",&num);

  flag=0;

  for(k=0;k<num+1;k++){ 


    while(( inp=getchar())!=EOF){
      

      inp-=ASK;

      if(inp=='\n'-ASK) {
	fl2=1;
	break;
      }
      tem=head;
      head=tail;
      tail=inp;

      if(inp==' '-ASK) {

	continue;
      }
      if(head==' '-ASK) {

	continue;
      }
      for(i=0;i<12;i++){
	if(head+ASK==kanglish[i+26][0]&&tail+ASK==kanglish[i+26][1])
	  {
	    field[tem][head]--;
	      tail=26+i;
	    flag=1;
	    //   printf("%s    %d \n",kanglish[i+26],tail);
	    if(tem>26)
	      {     
		// printf("!head and tail and out: %c %c %d %d %d\n",head+ASK,tail+ASK,head,tail,field[head][tail]);
		field[tem][tail]++;
		//		field[tem][head]--;
    
	      }
	    break;

	  }
      }

      if(fl2) 
	{

	  fl2=0;
continue;
	}
      if(flag){
	//      printf("flag: head and tail and out: %c %c %d %d %d\n",head+ASK,tail+ASK,head,tail,field[head][tail]);
	flag=0;
	continue;
      }

      field[head][tail]++;
      // printf("##head and tail and out: %c %c %d %d %d\n",head+ASK,tail+ASK,head,tail,field[head][tail]);
    }//while.



  }// end of input
  for(i=0;i<38;i++)
    ans[i]=0;
  for(i=0;i<38;i++){ 
    for(j=0;j<38;j++)
      if((field[i][j]>field[i][ans[i]]))
	ans[i]=j;
  }

  /* 
  for(i=0;i<38;i++) {
       printf("\n %c",i+ASK);
    for(j=0;j<38;j++)
    printf(" %d",field[i][j]);
  }
  */
    //output
  for(i=0;i<38;i++){
    if(field[i][ans[i]])
      printf("%s %s %d\n",kanglish[i],kanglish[ans[i]],field[i][ans[i]]);
    else   printf("%s %s %d\n",kanglish[i],kanglish[0],0);
  }
  return 0;
}