#pragma warning(disable:4996)
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>
#define MAX_STRING 20
#define MAXSHIPS 200000

	int aShipStart[MAXSHIPS];
	int aShipEnd[MAXSHIPS];

int main() {
	char sInput[MAX_STRING]="";
	char* psInput;
	int nMaxIsland=0;
	int nShipNumber=0;
	int nMiddleIsland=0;
	int i=0;
	int j=0;
	int nFind=0;
	
	// read line 1
	gets(sInput);
	psInput = strtok(sInput," ");
	nMaxIsland = atoi(psInput);
	psInput = strtok(NULL," ");
	nShipNumber = atoi(psInput);

	// read line 2
	for (i=0;i<nShipNumber;i++){
		gets(sInput);
		psInput = strtok(sInput," ");
		aShipStart[i] = atol(psInput);
		psInput = strtok(NULL," ");
		aShipEnd[i] = atol(psInput);
	}
	// search
	for (i=0;i<nShipNumber;i++){
		if (aShipStart[i]==1){
			nMiddleIsland = aShipEnd[i];
			for (j=0;j<nShipNumber;j++){
				if(aShipStart[j]==nMiddleIsland && aShipEnd[j]==nMaxIsland){
					 nFind=1;
					 break;
				}
			}
			if(nFind==1){
				break;
			}
		}
	}

	if (nFind==1){
		printf("POSSIBLE\n");
	} else {
		printf("IMPOSSIBLE\n");
	}

    return 0;
}
