#pragma warning(disable:4996)
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#define MAX_STRING 2200000 
#define MAXCARDS 100000

double alCards[MAXCARDS];
char sInput[MAX_STRING];

int main() {

	char* psInput;
	long long l=0;
	long long lTemp2=0;
/*	double  lLoopEnd=0.0;
	double  lTemp=0.0;
	double  lAllSum=0.0;
	double  lLeftSum=0.0;
	double  lRightSum=0.0;
	double  lTempMin=0.0;
	double  lMin=0.0; */
	long long  lCardsNumber =0;
	long long  lMax = 0;
	long long  lTop = 0;
	long long  lSecond = 0;
	long long  lTarget=0;
	long long  lFound=0;

	// read line 1
	gets(sInput);
	lCardsNumber = atol(sInput);
	//format
	for (l=0;l<lCardsNumber;l++){
		 alCards[l]=0;
	}
	// read line 2
	gets(sInput);
	psInput = strtok(sInput," ");
	for (l=0;l<lCardsNumber;l++){
		lTemp2=atol(psInput);
		alCards[lTemp2-1]=alCards[lTemp2-1]+1;
		if (lMax<lTemp2){
			lMax=lTemp2;
		}
		psInput = strtok(NULL," ");
	}

	for (lTarget=lMax;lTarget>0;lTarget--){
		if(alCards[lTarget-1]>=2){
			if(lTop==0){
				lTop=lTarget;
			} else {
				lSecond=lTarget;
				//lTarget=0;
				break;
			}
		}
	}

	printf("%ld\n",lTop*lSecond);

    return 0;
}
