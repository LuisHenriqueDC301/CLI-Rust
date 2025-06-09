#include <iostream>

bool Par(int number) {
    if(number % 2 == 0){
        return true;
    }
    return false;
}

int main() {
    bool x =  Par(5);

    std::cout << x << std::endl;

    return 0;
}