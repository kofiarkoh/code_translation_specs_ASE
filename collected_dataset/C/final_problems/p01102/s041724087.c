#include <stdio.h>

int main(){
	
	
	char str1[200],str2[200],judgement[3][10]={"IDENTICAL","CLOSE","DIFFERENT"};
	int count1,count2,flag1,flag2,judge;

	for(;;){

		judge=0;
		
		count1=0;
		count2=0;

		scanf("%s",str1);
		if(str1[0]=='.')break;
		scanf("%s",str2);

		for(;;){

			if(str1[count1]!=str2[count2])judge=2;

			if(str1[count1]=='\0' || str2[count2]=='\0')break;

//printf("aa");

			if(str1[count1]=='"'){///””の中

				count1++;
				count2++;
				flag1=0;
				flag2=0;

				for(;;){
					if(str1[count1]!=str2[count2]){
						if(judge==1){
							judge=2;
							break;
						}
						judge=1;
						break;
					}

					if(str1[count1]=='"')flag1=1;
					if(str2[count2]=='"')flag2=1;
					if(flag1+flag2>=1)break;
				
					count1++;
					count2++;

				}
				if(flag1==0)for(;str1[count1]!='"';count1++);
				if(flag2==0)for(;str2[count2]!='"';count2++);
			}

		if(judge==2)break;

		count1++;
		count2++;
		}

		printf("%s\n",judgement[judge]);
	}



	return 0;
}
