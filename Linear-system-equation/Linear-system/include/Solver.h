
/* 
 * File:   Solver.h
 * Author: Miguel Angel de la vega rodriguez
 *
 * Created on 3 de mayo de 2023, 21:52
 */

#ifndef SOLVER_H
#define SOLVER_H

#include <string>
#include <cmath>
#include <iostream>

class Solver{
public:
    /**
     * @brief Construct a new Solver object with the s
     * @param size 
     */
    Solver(int size = 0);

    /**
     * @brief Construct a new Solver object
     * 
     * @param other 
     */
    Solver(const Solver& other);

    /**
     * @brief Destroy the Solver object
     * 
     */
    ~Solver();

    /**
     * @brief Operator() used to access members of matrix_coef as if it was
     * a matrix: matrix_coef(i,j)
     * @param row 
     * @param col 
     * @return double* 
     */
    double* operator()(int row, int col);

    /**
     * @brief Set the Matrix Coef object
     * 
     * @param matrix_coef 
     * @param size 
     */
    void setMatrixCoef(double matrix_coef[], int size);
    void setMatrixInd(double ind[]);
    void gaussSeidel(int max_iterations, double tolerance);
    void solveSystem(void (*method));
    int tolerance(double tolerance);
    int inverseMatrix();
    std::string toString() const;
private:
    int rows;
    int cols;
    double* matrix_coef; // Matrix coefficients
    double* matrix_sol; // Solution vector
    double* matrix_ind; // Independent vector
};


#endif /* SOLVER_H */

